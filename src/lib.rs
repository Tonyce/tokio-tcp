use core::mem::MaybeUninit;
use std::ptr;
use tokio::runtime::{self, Runtime};

mod tcp;

pub static mut TOKIO_RT: MaybeUninit<runtime::Runtime> = MaybeUninit::uninit();

pub fn init_runtime() {
    unsafe {
        let rt = runtime::Runtime::new().unwrap();
        ptr::write(TOKIO_RT.as_mut_ptr(), rt);
    }
}

pub fn tcp_2_server() {
    let tokio_runtime: &Runtime = unsafe {
        let tokio = &mut *TOKIO_RT.as_mut_ptr();
        tokio
    };
    tokio_runtime.spawn(async {
        tcp::connect().await.unwrap();
    });
}

pub fn say_hello() {
    println!("hello world");
}
