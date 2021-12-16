use anyhow::Result;
use log::debug;
use std::net::TcpStream;
use sgx_crypto::tls_psk::*;

use manager_net::Command;

use crate::config::Config;
use crate::error::Error;
use crate::handlers::*;

pub fn execute(config : Config, request : Command, data : &str) -> Result<()> {
    // connect to Attestation Manager
    let stream = TcpStream::connect((config.host, config.port))?;
    let mut ctx = new_client_context(&config.key, stream)?;

    debug!("Connected to AttestationManager. Executing command: {:?}", request);

    match request {
        Command::Init               => handler_init(&mut ctx, request, data),
        Command::InitSGX            => handler_init_sgx(&mut ctx, request, data),
        Command::AttestSGX          => handler_attest_sgx(&mut ctx, request, data),
        Command::AttestNative       => handler_attest_native(&mut ctx, request, data),
        Command::AttestSancus       => handler_attest_sancus(&mut ctx, request, data),
        Command::AttestTrustZone    => handler_attest_trustzone(&mut ctx, request, data),
        Command::GetKey             => handler_get_key(&mut ctx, request, data),
        Command::GetPubKey          => handler_get_pub_key(&mut ctx, request, data),
        Command::Reset              => handler_reset(&mut ctx, request, data),
        _                           => return Err(Error::HandlerNotImplemented.into())
    }
}
