use tokio::net::{self, TcpStream};
// use async_std::prelude::*;
// use async_std::sync::Arc;
// use async_std::{future, task};
use futures::future::{self, BoxFuture, FutureExt};
// use futures::select;
use futures::future::poll_fn;
use std::error::Error;
use std::sync::Arc;
use std::time::Duration;
use tokio::io::{self, AsyncWriteExt, ReadHalf};
use tokio::prelude::*;
use tokio::sync::Mutex;
use tokio::time::delay_for;

fn connect_to_server(
    times: i32,
    status_cb: impl Fn(&str) + Send + Sync + 'static,
) -> BoxFuture<'static, Option<TcpStream>> {
    async move {
        match net::TcpStream::connect("127.0.0.1:1337").await {
            Ok(stream) => {
                status_cb("connected");
                return Some(stream);
            }
            Err(e) => {
                eprintln!("{}", e);
                // status_cb("connect fail...");
                delay_for(Duration::from_millis(1_000)).await;
                let msg = format!("retry connect...{}", times);
                status_cb(&msg);
                if times > 10 {
                    return None;
                }
                let times = times + 1;
                // let next_times = Arc::new(time);
                return connect_to_server(times, status_cb).await;
            }
        }
    }
    .boxed()
}

pub async fn connect() -> Result<(), Box<dyn Error>> {
    // Open a TCP stream to the socket address.
    //
    // Note that this is the Tokio TcpStream, which is fully async.
    let times = 0;
    let stream: Option<TcpStream> = connect_to_server(times, |msg| {
        println!("msg {}", msg);
    })
    .await;

    if let Some(mut socket) = stream {
        println!("created stream");
        // let mut buffer = vec![0u8; 1000]; //reserve 1000 bytes in the receive buffer
        //get all data that is available to us at the moment...

        // let (mut read_half, mut write_half) = stream.split();
        // let (mut recv, mut send) = io::split(stream);
        // let mut stdin = io::stdin();
        // let mut stdout = io::stdout();

        // let send = tokio::spawn(async move { io::copy(&mut stdin, &mut send).await });
        // let recv = tokio::spawn(async move { io::copy(&mut recv, &mut stdout).await });

        // send.await??;
        // recv.await??;

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            // In a loop, read data from the socket and write the data back.
            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => return,
                    Ok(n) => n,
                    Err(e) => {
                        eprintln!("failed to read from socket; err = {:?}", e);
                        return;
                    }
                };

                // Write the data back
                if let Err(e) = socket.write_all(&buf[0..n]).await {
                    eprintln!("failed to write to socket; err = {:?}", e);
                    return;
                }
            }
        });
    // let result = stream.write(b"hello world\n").await;
    // println!("wrote to stream; success={:?}", result.is_ok());
    } else {
        println!("connot connet, wait a memont...");
    }

    Ok(())
}
