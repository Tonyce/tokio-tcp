use core::mem::MaybeUninit;
use std::ptr;
use tokio::runtime::{self, Runtime};
use tokio::net::TcpStream;
use futures::lock::Mutex;
use std::sync::Arc;
use tokio::io::{self, AsyncWriteExt, ReadHalf, WriteHalf};
use std::io::prelude::*;
use tokio::prelude::*;
use tokio::sync::mpsc;

mod tcp;

pub static mut TOKIO_RT: MaybeUninit<runtime::Runtime> = MaybeUninit::uninit();

// static my_channel: (mpsc::Sender<String>, mpsc::Receiver<String>) = mpsc::channel(100);

pub fn init_runtime() {
    unsafe {
        let rt = runtime::Runtime::new().unwrap();
        ptr::write(TOKIO_RT.as_mut_ptr(), rt);
    }
}

pub struct TcpClient {
    tcpStream: Option<TcpStream>,
    tx: Option<mpsc::Sender<String>>,
    tcpDownReciver: Option<mpsc::Receiver<i32>>,
    // rx: mpsc::Receiver<String>,
}

impl TcpClient {
    pub fn new() -> Self {
        TcpClient {
            tx: None,
            tcpStream: None,
            // rx,
            // writer: None,
        }
    }

    pub fn connect_to_server(&mut self, callback: impl Fn(&str)) {
        let tokio_runtime: &Runtime = unsafe {
            let tokio = &mut *TOKIO_RT.as_mut_ptr();
            tokio
        };

        // let client = Arc::new(Mutex::new(self));
        // let client = Arc::new(self);

        // let mut client_copy = client.clone();
        let (tx, mut rx) = mpsc::channel(100);
        self.tx = Some(tx);

        let (tcpDownSender, mut tcpDownReciver) = mpsc::channel(100);
        self.tcpDownReciver = Some(tcpDownReciver);

        self.worke();

        tokio_runtime.spawn(async move {
            let stream: Option<TcpStream> = tcp::connect_to_server(10 as i32, |msg| {
                println!("msg {}", msg);
            })
            .await;
            if let Some(mut stream) = stream {
                let (mut read_half, mut write_half) = io::split(stream);

                tokio_runtime.spawn(async move {
                    let mut buf = [0; 1024];
        
                    // In a loop, read data from the socket and write the data back.
                    loop {
                        let _n = match read_half.read(&mut buf).await {
                            // socket closed
                            Ok(n) if n == 0 => {
                                tcpDownSender.send(0).await.unwrap();
                                println!("socket closed");
                                return;
                            },
                            Ok(n) => {
                                println!("n {}", n);
                            },
                            Err(e) => {
                                eprintln!("failed to read from socket; err = {:?}", e);
                                return;
                            }
                        };
                    }
                });

                while let Some(res) = rx.recv().await {
                    write_half.write(res.as_bytes()).await;
                    println!("writer...");
                }
            }
        });
        println!("connect_to_server");
        // client
        // self.tcpStream = tcpStream;
    }

    fn worke(&self) {
        let tokio_runtime: &Runtime = unsafe {
            let tokio = &mut *TOKIO_RT.as_mut_ptr();
            tokio
        };
        
        if let Some(tcpDownReciver) = self.tcpDownReciver {
            let mut tx = tcpDownReciver.clone();

        };
    }

    pub fn send_msg(&self) {
        let tokio_runtime: &Runtime = unsafe {
            let tokio = &mut *TOKIO_RT.as_mut_ptr();
            tokio
        };

        if let Some(tx) = self.tx.as_ref() {
            let mut tx = tx.clone();
            tokio_runtime.spawn(async move {
                // tx.send(&b"data to write"[..]).await.unwrap();
                tx.send("send".to_owned()).await.unwrap();
            });
        }
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
