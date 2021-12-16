use std::net::TcpStream;
use std::io::Write;
use sgx_crypto::tls_psk::*;

#[test]
#[ignore]
fn test_tls_native() {
    let key : [u8; 16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    let host = "0.0.0.0";
    let port = 1234;
    let msg = b"hello";

    let stream = TcpStream::connect((host, port)).expect("Failed to connect to server");
    let mut ctx = new_client_context(&key, stream).expect("Failed to establish TLS connection");

    println!("Established TLS connection");
    ctx.write(msg).expect("Failed to write msg");

    println!("Completed");
}
