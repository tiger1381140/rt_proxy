use libc::{c_int, getsockopt, sockaddr_in, socklen_t};
use std::mem;
use std::os::unix::io::AsRawFd;
use tokio::net::TcpStream;

const SOL_IP: c_int = 0; // 获取原始目的地址的选项
const SO_ORIGINAL_DST: c_int = 80; // 获取原始目的地址的选项

pub fn common_get_orig_dst(
    down_socket: &TcpStream,
) -> Result<std::net::SocketAddr, std::io::Error> {
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
