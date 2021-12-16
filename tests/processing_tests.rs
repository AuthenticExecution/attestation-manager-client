use std::net::TcpStream;
use sgx_crypto::tls_psk::*;
use manager_net::{Command, InitData, InitResponse, ErrorResponse, write_command, read_response};
use sgx_crypto::Context;

#[test]
#[ignore]
pub fn test_init() {
    let key : [u8; 16] = [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16];
    let host = "0.0.0.0";
    let port = 1234;

    let stream = TcpStream::connect((host, port)).expect("Failed to connect to server");
    let mut ctx = new_client_context(&key, stream).expect("Failed to establish TLS connection");


    println!("Established TLS connection");

    let init_data = InitData::new();

    write_command(&mut ctx, Command::Init.to_u8(), &init_data).unwrap();
    println!("Data sent");

    let _resp = read_response::<Context, InitResponse, ErrorResponse>(&mut ctx).unwrap().unwrap();
    println!("Completed");
}
