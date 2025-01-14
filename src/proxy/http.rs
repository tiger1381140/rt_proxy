use libc::{c_int, getsockopt, sockaddr_in, socklen_t};
use std::mem;
use std::os::unix::io::AsRawFd;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

const SOL_IP: c_int = 0; // 获取原始目的地址的选项
const SO_ORIGINAL_DST: c_int = 80; // 获取原始目的地址的选项

pub enum UpDownBuffer {
    _Up(Vec<u8>),
    _Down(Vec<u8>),
}

pub struct _FiveInfo {
    pub _src_ipv4: u32,
    pub _dst_ipv4: u32,
    pub _src_port: u16,
    pub _dst_port: u16,
    pub _protocol: u8,
}

pub struct Http {
    pub _icap_socket: TcpStream,
    pub _down_buffer: Vec<Vec<u8>>,
    pub _icap_buffer: Vec<Vec<u8>>,
    pub _up_buffer: Vec<Vec<u8>>,
}

impl Http {
    pub fn new(icap_socket: TcpStream) -> Self {
        Self {
            _icap_socket: icap_socket,
            _down_buffer: Vec::new(),
            _icap_buffer: Vec::new(),
            _up_buffer: Vec::new(),
        }
    }

    fn get_orig_dst(down_socket: &TcpStream) -> Result<std::net::SocketAddr, std::io::Error> {
        let mut addr: sockaddr_in = unsafe { mem::zeroed() };
        let mut addr_len = mem::size_of::<sockaddr_in>() as socklen_t;

        let ret = unsafe {
            getsockopt(
                down_socket.as_raw_fd(),
                SOL_IP,
                SO_ORIGINAL_DST,
                &mut addr as *mut _ as *mut _,
                &mut addr_len,
            )
        };

        if ret == -1 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "获取原始目的地址失败",
            ));
        }

        let ip = std::net::Ipv4Addr::from(u32::from_be(addr.sin_addr.s_addr));
        let port = u16::from_be(addr.sin_port);
        Ok(std::net::SocketAddr::new(std::net::IpAddr::V4(ip), port))
    }

    fn read_service_up(&mut self, buffer: &mut [u8], size: usize) -> Option<Vec<u8>> {
        return Some(buffer[0..size].to_vec());
    }

    fn read_service_down(&mut self, buffer: &mut [u8], size: usize) -> Option<Vec<u8>> {
        return Some(buffer[0..size].to_vec());
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
        let icap_socket = TcpStream::connect("127.0.0.1:1344").await?;

        let orig_dst = Self::get_orig_dst(&down_socket)?;
        let mut up_socket = TcpStream::connect(orig_dst).await?;

        let mut http = Http::new(icap_socket);
        let mut buffer_down = [0u8; 8192];
        let mut buffer_up = [0u8; 8192];

        loop {
            tokio::select! {
                up_socket_msg = up_socket.read(&mut buffer_up) => {
                    match up_socket_msg {
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

                down_socket_msg = down_socket.read(&mut buffer_down) => {
                    match down_socket_msg {
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
