use crate::common::common_net::common_get_orig_dst;
use crate::protocol::http::ProtoHttpCtx;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub enum UpDownBuffer {
    _Up(Vec<u8>),
    _Down(Vec<u8>),
}

pub struct Http {
    pub head_down_buffer: Vec<u8>,
    pub body_down_buffer: Vec<u8>,

    pub head_up_buffer: Vec<u8>,
    pub body_up_buffer: Vec<u8>,

    pub _icap_buffer: Vec<u8>,
    pub http_ctx: ProtoHttpCtx,
}

impl Http {
    pub fn new() -> Self {
        Self {
            head_down_buffer: Vec::new(),
            body_down_buffer: Vec::new(),

            head_up_buffer: Vec::new(),
            body_up_buffer: Vec::new(),
            _icap_buffer: Vec::new(),
            http_ctx: ProtoHttpCtx::new(),
        }
    }

    fn read_service_up(&mut self, buffer: &mut [u8], size: usize) -> Option<Vec<u8>> {
        // 如果不符合不合法，则将数据发生给http client端
        if !self.http_ctx.is_valid() {
            return Some(buffer[0..size].to_vec());
        }

        // 如果已经解析了请求头，则将数据推入到head_up_buffer中, 延迟发送给icap server端
        if self.http_ctx.resp_seen_head() {
            self.http_ctx.resp_seen_bytes_inc(size as u64);
            self.body_up_buffer.extend_from_slice(buffer);
            return None;
        }

        // 后续数据不能使用buffer；而要使用head_up_buffer
        self.head_up_buffer.extend_from_slice(buffer);

        let head_size = self.http_ctx.parse_http_resp_header(&self.head_up_buffer);
        // 如果不合法，则将数据发生给http client端
        if !self.http_ctx.is_valid() {
            return Some(self.head_up_buffer.drain(..).collect());
        }

        // 如果已经解析了请求头，则将数据推入到body_up_buffer中, 延迟发生给icap server端
        if self.http_ctx.resp_seen_head() {
            self.body_up_buffer
                .extend(self.head_up_buffer.drain(head_size..));
            return None;
        }

        // 长度不够，继续收包
        assert!(head_size == 0, "head_size is 0");
        return None;
    }

    fn read_service_down(&mut self, buffer: &mut [u8], size: usize) -> Option<Vec<u8>> {
        // 如果不符合不合法，则将数据发生给http server端
        if !self.http_ctx.is_valid() {
            return Some(buffer[0..size].to_vec());
        }

        // 如果已经解析了请求头，则将数据推入到body_down_buffer中, 延迟发送给icap server端
        if self.http_ctx.req_seen_head() {
            self.http_ctx.req_seen_bytes_inc(size as u64);
            self.body_down_buffer.extend_from_slice(buffer);
            return None;
        }

        // 后续数据不能使用buffer；而要使用head_down_buffer
        self.head_down_buffer.extend_from_slice(buffer);

        let head_size = self.http_ctx.parse_http_req_header(&self.head_down_buffer);
        // 如果不合法，则将数据发生给http server端
        if !self.http_ctx.is_valid() {
            return Some(self.head_down_buffer.drain(..).collect());
        }

        // 如果已经解析了请求头，则将数据推入到body_down_buffer中, 延迟发送给icap server端
        if self.http_ctx.req_seen_head() {
            self.body_down_buffer
                .extend(self.head_down_buffer.drain(head_size..));
            return None;
        }

        // 长度不够，继续收包
        assert!(head_size == 0, "head_size is 0");
        return None;
    }

    fn read_service_icap(&mut self, _buffer: &mut [u8], _size: usize) -> Option<UpDownBuffer> {
        return None;
    }

    async fn pending_service(&mut self) -> Option<UpDownBuffer> {
        return None;
    }

    pub async fn accept_service(
        http_listen: &Option<TcpListener>,
    ) -> Result<TcpStream, std::io::Error> {
        if http_listen.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "HTTP 监听器为空",
            ));
        }
        let listener = http_listen.as_ref().unwrap();
        let (socket, _) = listener.accept().await?;
        Ok(socket)
    }

    pub async fn process_service(mut down_socket: TcpStream) -> Result<(), std::io::Error> {
        let mut icap_socket = TcpStream::connect("127.0.0.1:1344").await?;

        let orig_dst = common_get_orig_dst(&down_socket)?;
        let mut up_socket = TcpStream::connect(orig_dst).await?;

        let mut http = Http::new();
        let mut buffer_down = [0u8; 8192];
        let mut buffer_up = [0u8; 8192];
        let mut buffer_icap = [0u8; 8192];
        loop {
            tokio::select! {
                msg = up_socket.read(&mut buffer_up) => {
                    match msg {
                        Ok(n) => {
                            if n == 0 { break; }
                            let msg = http.read_service_up(&mut buffer_up, n);
                            if msg.is_some() {
                                let msg = msg.unwrap();
                                down_socket.write_all(&msg).await?;
                            }
                        }
                        Err(e) => { return Err(e); }
                    }
                }

                msg = down_socket.read(&mut buffer_down) => {
                    match msg {
                        Ok(n) => {
                            if n == 0 { break; }
                            let msg = http.read_service_down(&mut buffer_down, n);
                            if msg.is_some() {
                                let msg = msg.unwrap();
                                up_socket.write_all(&msg).await?;
                            }
                        }
                        Err(e) => { return Err(e); }
                    }
                }

                msg = icap_socket.read(&mut buffer_icap) => {
                    match msg {
                        Ok(n) => {
                            if n == 0 { break; }
                            let msg = http.read_service_icap(&mut buffer_icap, n);
                            if msg.is_some() {
                                let msg = msg.unwrap();
                                match msg {
                                    UpDownBuffer::_Up(msg) => { up_socket.write_all(&msg).await?; }
                                    UpDownBuffer::_Down(msg) => { down_socket.write_all(&msg).await?; }
                                }
                            }
                        }
                        Err(e) => { return Err(e); }
                    }
                }

                msg = http.pending_service() => {
                    if let Some(msg) = msg {
                        match msg {
                            UpDownBuffer::_Up(msg) => { up_socket.write_all(&msg).await?; }
                            UpDownBuffer::_Down(msg) => { down_socket.write_all(&msg).await?; }
                        }
                    }
                }

            }
        }
        Ok(())
    }
}
