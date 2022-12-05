use anyhow::Result;
use log::{debug};
use serde::Deserialize;
use std::net::TcpStream;

use sgx_crypto::Context;
use manager_net::*;

use crate::utils::*;
use crate::error::Error;
use crate::structs::*;

// a sort of "specialization" of rust_net::read_response
pub fn read_response<T : for<'de> Deserialize<'de>>(session : &mut Context<TcpStream>) -> Result<Result<T, ErrorResponse>, manager_net::Error> {
    manager_net::read_response::<Context<TcpStream>, T, ErrorResponse>(session)
}

pub fn handler_init(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : InitData = read_from_file(data)?;
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<InitResponse>(session)?;

    match resp {
        Ok(_)   => Ok(()),
        Err(e)  => Err(Error::ServerError(e.description).into())
    }
}

pub fn handler_init_sgx(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : InitSGXDataFiles = read_from_file(data)?;
    debug!("{:?}", data);

    let data = data.to_sgx_data()?;

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<InitResponse>(session)?;

    match resp {
        Ok(_)   => Ok(()),
        Err(e)  => Err(Error::ServerError(e.description).into())
    }
}

pub fn handler_attest_sgx(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : AttestSGXFiles = read_from_file(data)?;
    debug!("{:?}", data);

    let data = data.to_sgx_data()?;

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<AttestationResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("{} attested", data.name);

    // print the key
    println!("{:?}", resp.module_key);

    Ok(())
}

pub fn handler_attest_native(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : AttestationRequestNative = read_from_file(data)?;
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<AttestationResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("{} attested", data.name);

    // print the key
    println!("{:?}", resp.module_key);

    Ok(())
}

pub fn handler_attest_sancus(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : AttestationRequestSancus = read_from_file(data)?;
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<AttestationResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("{} attested", data.name);

    // print the key
    println!("{:?}", resp.module_key);

    Ok(())
}

pub fn handler_attest_trustzone(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : AttestationRequestTrustZone = read_from_file(data)?;
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<AttestationResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("{} attested", data.name);

    // print the key
    println!("{:?}", resp.module_key);

    Ok(())
}


pub fn handler_get_key(session : &mut Context<TcpStream>, request : Command, data : &str) -> Result<()> {
    let data : GetKeyRequest = read_from_file(data)?;
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<GetKeyResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("Got key of {}", data.name);

    // print the key
    println!("{:?}", resp.module_key);

    Ok(())
}

pub fn handler_get_pub_key(session : &mut Context<TcpStream>, request : Command, _data : &str) -> Result<()> {
    let data = GetPubKeyRequest::new();
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<GetPubKeyResponse>(session)?;

    let resp = match resp {
        Ok(r)   => r,
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("Got SP public key");

    // print the key
    println!("{}", std::str::from_utf8(&resp.sp_pubkey)?);

    Ok(())
}

pub fn handler_reset(session : &mut Context<TcpStream>, request : Command, _data : &str) -> Result<()> {
    let data = ResetRequest::new();
    debug!("{:?}", data);

    write_command(session, request.to_u8(), &data)?;
    let resp = read_response::<ResetResponse>(session)?;

    match resp {
        Ok(_)   => (),
        Err(e)  => return Err(Error::ServerError(e.description).into())
    };

    debug!("Reset completed");

    Ok(())
}
