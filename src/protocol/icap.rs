use icaparse::{Response, Status};

pub struct ProtoIcapCtx {
    body: Vec<u8>,
    code: u16,
    seen_header: bool,
    not_vaild: bool,
}

impl ProtoIcapCtx {
    pub fn new() -> Self {
        Self {
            body: Vec::new(),
            code: 204,
            seen_header: false,
            not_vaild: false,
        }
    }

    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }
    pub fn get_body(&self) -> Vec<u8> {
        self.body.clone()
    }

    pub fn set_code(&mut self, code: u16) {
        self.code = code;
    }
    pub fn get_code(&self) -> u16 {
        self.code
    }

    pub fn set_seen_head(&mut self, seen: bool) {
        self.seen_header = seen;
    }
    pub fn get_seen_head(&self) -> bool {
        self.seen_header
    }

    pub fn get_vaild(&self) -> bool {
        !self.not_vaild
    }
    pub fn set_valid(&mut self, valid: bool) {
        self.not_vaild = !valid;
    }
    
    pub fn reset(&mut self) {
        self.body.clear();
        self.code = 204;
        self.seen_header = false;
        self.not_vaild = false;
    }

    pub fn parse_icap_resp(&mut self, data: &[u8]) -> usize {
        let mut headers = [icaparse::EMPTY_HEADER; 32];
        let mut res = Response::new(&mut headers);
        match res.parse(data) {
            Ok(Status::Complete(header_end)) => {
                println!("ICAP Response Parsed Successfully!");
                println!("Version: {}", res.version.unwrap());
                println!("Status: {}", res.code.unwrap());
                println!("Reason: {}", res.reason.unwrap());
                for header in res.headers {
                    println!("Header: {} => {}", header.name, String::from_utf8_lossy(&header.value));
                }
                self.set_code(res.code.unwrap());
                self.set_body(data[header_end..].to_vec());
                self.set_seen_head(true);
                return header_end;
            }
            Ok(Status::Partial) => {
                println!("ICAP Response is incomplete. Waiting for more data...");
                return 0;
            }
            Err(e) => {
                println!("Error parsing ICAP response: {}", e);
                self.set_valid(false);
                return 0;
            }
        }
    }

}
