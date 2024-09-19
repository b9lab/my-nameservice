use cosmwasm_std::{Addr, StdError};
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),
    #[error("Name already taken ({name})")]
    NameTaken { name: String },
    #[error("Caller ({caller}) is not minter")]
    Minter {
        caller: String,
        inner: OwnershipError,
    },
}

impl ContractError {
    pub fn from_minter<'a>(caller: &'a Addr) -> impl Fn(OwnershipError) -> ContractError + 'a {
        move |inner: OwnershipError| ContractError::Minter {
            caller: caller.to_string(),
            inner,
        }
    }
}
