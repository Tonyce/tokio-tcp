use futures::lock::Mutex;
use std::sync::Arc;
use client_sdk_tokio;
use std::thread;
use std::time;

fn main() {
    client_sdk_tokio::say_hello();
    client_sdk_tokio::init_runtime();
    // client_sdk_tokio::tcp_2_server();
    // let client: Arc<Mutex<client_sdk_tokio::TcpClient>> = client_sdk_tokio::TcpClient::new().connect_to_server();
    let mut client = client_sdk_tokio::TcpClient::new();
    client.connect_to_server(|_| {

    });

    loop {
        thread::sleep(time::Duration::from_millis(500));
        client.send_msg();
    }
}
