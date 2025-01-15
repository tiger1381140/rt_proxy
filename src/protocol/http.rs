use httparse::{Request, Response, Status};

struct ProtoHttpReq {
    pub seen_header: bool,
    pub http_method: String,
    pub seen_bytes: u64,
}

struct ProtoHttpResp {
    pub seen_header: bool,
    pub http_status_code: u16,
    pub seen_bytes: u64,
}
pub struct ProtoHttpCtx {
    pub not_valid: bool,
    req: ProtoHttpReq,
    resp: ProtoHttpResp,
}

impl ProtoHttpReq {
    pub fn new() -> Self {
        Self {
            seen_header: false,
            http_method: String::new(),
            seen_bytes: 0,
        }
    }
}

impl ProtoHttpResp {
    pub fn new() -> Self {
        Self {
            seen_header: false,
            http_status_code: 0,
            seen_bytes: 0,
        }
    }
}

impl ProtoHttpCtx {
    pub fn new() -> Self {
        Self {
            not_valid: false,
            req: ProtoHttpReq::new(),
            resp: ProtoHttpResp::new(),
        }
    }

    pub fn req_seen_bytes_inc(&mut self, size: u64) {
        self.req.seen_bytes += size;
    }
    pub fn _req_seen_bytes(&self) -> u64 {
        self.req.seen_bytes
    }
    pub fn req_seen_head_set(&mut self, seen: bool) {
        self.req.seen_header = seen;
    }
    pub fn req_seen_head(&self) -> bool {
        self.req.seen_header
    }

    pub fn resp_seen_bytes_inc(&mut self, size: u64) {
        self.resp.seen_bytes += size;
    }
    pub fn _resp_seen_bytes(&self) -> u64 {
        self.resp.seen_bytes
    }
    pub fn resp_seen_head_set(&mut self, seen: bool) {
        self.resp.seen_header = seen;
    }
    pub fn resp_seen_head(&self) -> bool {
        self.resp.seen_header
    }

    pub fn _req_method(&self) -> String {
        self.req.http_method.clone()
    }

    pub fn is_valid(&self) -> bool {
        !self.not_valid
    }

    /*  解析请求头
     * 解析成功，返回解析到的字节数；
     * 如果解析失败，则返回0 且 设置http非法
     * 如果数据不够，则返回0
     * */
    pub fn parse_http_req_header(&mut self, data: &[u8]) -> usize {
        self.resp_seen_head_set(false);

        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut req = Request::new(&mut headers);

        // 解析 Header
        match req.parse(data) {
            Ok(Status::Complete(header_end)) => {
                self.req.http_method = req.method.unwrap().to_string();
                println!("Request Headers parsed successfully:");
                println!("Method: {}", self.req.http_method);
                println!("Path: {}", req.path.unwrap());
                for header in req.headers {
                    println!(
                        "Header: {} => {}",
                        header.name,
                        String::from_utf8_lossy(header.value)
                    );
                }
                self.req_seen_head_set(true);
                self.req_seen_bytes_inc(data.len() as u64);
                return header_end;
            }
            Ok(Status::Partial) => {
                println!("Incomplete request headers. Waiting for more data...");
                return 0;
            }
            Err(e) => {
                println!("Failed to parse request: {:?}", e);
                self.not_valid = true;
                return 0;
            }
        }
    }

    /*  解析响应头
     * 解析成功，返回解析到的字节数；
     * 如果解析失败，则返回0 且 设置http非法
     * 如果数据不够，则返回0
     * */
    pub fn parse_http_resp_header(&mut self, data: &[u8]) -> usize {
        self.req_seen_head_set(false);

        // 定义存储 HTTP 头部的数组
        let mut headers = [httparse::EMPTY_HEADER; 16];
        let mut res = Response::new(&mut headers);

        // 解析 Header
        match res.parse(data) {
            Ok(Status::Complete(header_end)) => {
                println!("Response Headers parsed successfully:");
                self.resp.http_status_code = res.code.unwrap();
                println!("Response Status: {}", self.resp.http_status_code);

                for header in res.headers {
                    println!(
                        "Header: {} => {}",
                        header.name,
                        String::from_utf8_lossy(header.value)
                    );
                }
                self.resp_seen_head_set(true);
                self.resp_seen_bytes_inc(data.len() as u64);
                return header_end;
            }
            Ok(Status::Partial) => {
                println!("Incomplete headers. Waiting for more data...");
                return 0;
            }
            Err(e) => {
                println!("Failed to parse response: {:?}", e);
                self.not_valid = true;
                return 0;
            }
        }
    }
}
