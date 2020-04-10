use client_sdk_tokio;

fn main() {
    client_sdk_tokio::say_hello();
    client_sdk_tokio::init_runtime();
    client_sdk_tokio::tcp_2_server();
    // let mut over = false;

    loop {}
}
