use serde::{Serialize, Deserialize};
use std::fs;
use anyhow::Result;

use manager_net::{InitSGXData, AttestationRequestSGX};

#[derive(Serialize, Deserialize, Debug)]
pub struct InitSGXDataFiles {
    pub sp_privkey : String,
    pub sp_pubkey : String,
    pub ias_certificate : String
}

impl InitSGXDataFiles {
    pub fn to_sgx_data(&self) -> Result<InitSGXData> {
        let mut sp_privkey = fs::read(&self.sp_privkey)?;
        let mut sp_pubkey = fs::read(&self.sp_pubkey)?;
        let mut ias_certificate = fs::read(&self.ias_certificate)?;

        // fix bug with PEM keys
        sp_privkey.push(0);
        sp_pubkey.push(0);
        ias_certificate.push(0);

        Ok(InitSGXData::new(sp_privkey, sp_pubkey, ias_certificate))
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AttestSGXFiles {
    pub id : u16,
    pub name : String,
    pub host : String,
    pub port : u16,
    pub em_port : u16,
    pub aesm_client_host : String,
    pub aesm_client_port : u16,
    pub sigstruct : String,
    pub config : String
}

impl AttestSGXFiles {
    pub fn to_sgx_data(self) -> Result<AttestationRequestSGX> {
        let sigstruct = fs::read(&self.sigstruct)?;
        let config = fs::read(&self.config)?;

        Ok(AttestationRequestSGX::new(self.id, self.name, self.host, self.port, self.em_port, self.aesm_client_host, self.aesm_client_port, sigstruct, config))
    }
}
