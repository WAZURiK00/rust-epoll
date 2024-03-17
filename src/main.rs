use std::{
    ffi::{c_int, c_long, c_uint},
    io::{Read, Write},
    os::fd::AsRawFd,
    process::exit,
};

#[link(name = "c")]
extern "C" {
    pub fn epoll_create1(fd: c_int) -> c_int;
    pub fn epoll_ctl(epfd: c_int, op: c_int, fd: c_int, event: *mut Event) -> c_int;
    pub fn epoll_wait(epfd: c_int, events: *mut Event, max_events: c_int, timeout: c_int) -> c_int;
}

#[derive(Debug)]
#[repr(C, packed)]
pub struct Event {
    pub events: c_uint,
    pub data: c_long,
}

const EPOLL_CTL_ADD: i32 = 1;
const EPOLLIN: i32 = 0x1;
const EPOLLET: i32 = 1 << 31;

fn get_req(path: &str) -> String {
    format!(
        "GET {path} HTTP/1.1\r\n\
         Host: localhost\r\n\
         Connection: close\r\n\
         \r\n"
    )
}

fn main() {
    let epoll_instance = unsafe { epoll_create1(0) };
    if epoll_instance == -1 {
        println!("Error happened");
        exit(-1);
    }

    let request = get_req("/5000/request-5");
    let mut stream = std::net::TcpStream::connect("localhost:8080").unwrap();
    stream.set_nonblocking(true).unwrap();

    stream.write_all(request.as_bytes()).unwrap();

    let res = unsafe {
        let mut event = Event {
            events: (EPOLLIN | EPOLLET) as u32,
            data: 5,
        };
        epoll_ctl(
            epoll_instance,
            EPOLL_CTL_ADD,
            stream.as_raw_fd(),
            &mut event,
        )
    };
    if res == -1 {
        println!("Error occurred registering stream in epoll");
        exit(-1);
    }

    let mut data = [0u8; 4096];
    let mut events: Vec<Event> = Vec::with_capacity(1);
    unsafe {
        let res = epoll_wait(epoll_instance, events.as_mut_ptr(), 1, -1);
        if res == 0 {
            println!("False alarm");
            exit(-1);
        }

        events.set_len(res as usize);
    };

    match stream.read(&mut data) {
        Ok(n) => {
            let txt = String::from_utf8_lossy(&data[..n]);
            println!("{txt}");
        }
        Err(e) => {
            println!("Error happened");
            println!("{}", e);
        }
    }
}
