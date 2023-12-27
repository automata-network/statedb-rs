use std::prelude::v1::*;

use base::trace::AvgCounterResult;
use eth_types::{FetchState, FetchStateResult, HexBytes, StateAccountTrait, SH160, SH256, SU256};
use std::borrow::Cow;
use std::fmt::Debug;
use std::sync::Arc;

#[derive(Debug)]
pub enum Error {
    DecodeError(rlp::DecoderError),
    CodeNotFound(SH256),
    WithKey(String),
    CallRemoteFail(String),
    Flush(String),
}

#[derive(Debug, Default)]
pub struct MissingState {
    pub code: bool,
    pub account: bool,
    pub storages: Vec<SH256>,
}

pub trait StateDB {
    type StateAccount: StateAccountTrait;
    fn fork(&self) -> Self;
    fn suicide(&mut self, address: &SH160) -> Result<(), Error>;
    fn get_state(&mut self, address: &SH160, index: &SH256) -> Result<SH256, Error>;
    fn exist(&mut self, address: &SH160) -> Result<bool, Error>;
    fn get_balance(&mut self, address: &SH160) -> Result<SU256, Error>;
    fn state_root(&self) -> SH256;
    fn flush(&mut self) -> Result<SH256, Error>;
    fn revert(&mut self, root: SH256);
    fn try_get_acc(&mut self, address: &SH160) -> Result<Option<Self::StateAccount>, Error>;
    fn get_code(&mut self, address: &SH160) -> Result<Arc<HexBytes>, Error>;
    fn set_code(&mut self, address: &SH160, code: Vec<u8>) -> Result<(), Error>;
    fn get_nonce(&mut self, address: &SH160) -> Result<u64, Error>;
    fn set_nonce(&mut self, address: &SH160, val: SU256) -> Result<(), Error>;
    fn sub_balance(&mut self, address: &SH160, val: &SU256) -> Result<(), Error>;
    fn set_state(&mut self, address: &SH160, index: &SH256, value: SH256) -> Result<(), Error>;
    fn add_balance(&mut self, address: &SH160, val: &SU256) -> Result<(), Error>;
    fn set_balance(&mut self, address: &SH160, val: SU256) -> Result<(), Error>;
    fn try_get_nonce(&mut self, address: &SH160) -> Option<u64>;
    fn get_account_basic(&mut self, address: &SH160) -> Result<(SU256, u64), Error>;
    fn apply_states(&mut self, result: Vec<FetchStateResult>) -> Result<(), Error>;
    fn check_missing_state(
        &mut self,
        address: &SH160,
        storages: &[SH256],
    ) -> Result<MissingState, Error>;
}

pub trait Trie {
    type DB: NodeDB;
    fn root_hash(&self) -> SH256;
    fn try_get(&self, db: &mut Self::DB, key: &[u8]) -> Option<Vec<u8>>;
    fn get(&self, db: &mut Self::DB, key: &[u8]) -> Result<Vec<u8>, String>;
    fn update(&mut self, db: &mut Self::DB, updates: Vec<(&[u8], Vec<u8>)>) -> Vec<TrieUpdate>;
    fn new_root(&self, new_root: SH256) -> Self;
}

#[derive(Clone, Debug)]
pub enum TrieUpdate {
    Success,
    Missing(SH256),
}

pub trait NodeDB {
    type Node;
    fn fork(&self) -> Self;
    fn get(&self, index: &SH256) -> Option<Arc<Self::Node>>;
    fn add_node(&mut self, node: &Arc<Self::Node>);

    fn get_code(&mut self, hash: &SH256) -> Option<Arc<HexBytes>>;
    fn set_code(&mut self, hash: SH256, code: Cow<HexBytes>);

    fn remove_staging_node(&mut self, node: &Arc<Self::Node>);
    fn staging(&mut self, node: Self::Node) -> Arc<Self::Node>;
    fn commit(&mut self) -> usize;
}

pub trait ProofFetcher {
    fn fetch_proofs(&self, key: &[u8]) -> Result<Vec<HexBytes>, String>;
    fn get_nodes(&self, node: &[SH256]) -> Result<Vec<HexBytes>, String>;
}

pub trait StateFetcher: ProofFetcher {
    fn with_acc(&self, address: &SH160) -> Self;
    fn get_block_hash(&self, number: u64) -> Result<SH256, Error>;
    fn get_code(&self, address: &SH160) -> Result<HexBytes, Error>;
    fn get_account(&self, address: &SH160) -> Result<(SU256, u64, HexBytes), Error>;
    fn get_storage(&self, address: &SH160, key: &SH256) -> Result<SH256, Error>;
    fn fork(&self) -> Self;
    fn get_miss_usage(&self) -> AvgCounterResult;
    fn prefetch_states(
        &self,
        list: &[FetchState],
        with_proof: bool,
    ) -> Result<Vec<FetchStateResult>, Error>;
}

pub type NoStateFetcher = ();

impl ProofFetcher for NoStateFetcher {
    fn fetch_proofs(&self, key: &[u8]) -> Result<Vec<HexBytes>, String> {
        Err(format!("key not found for proofs: {:?}", key))
    }

    fn get_nodes(&self, node: &[SH256]) -> Result<Vec<HexBytes>, String> {
        Err(format!("nodes not found: {:?}", node))
    }
}

impl StateFetcher for NoStateFetcher {
    fn fork(&self) -> Self {
        ()
    }

    fn get_account(&self, address: &SH160) -> Result<(SU256, u64, HexBytes), Error> {
        Err(Error::WithKey(format!("account[{:?}] not found", address)))
    }

    fn get_block_hash(&self, number: u64) -> Result<SH256, Error> {
        Err(Error::WithKey(format!(
            "block_hash[{:?}] not found",
            number
        )))
    }

    fn get_code(&self, address: &SH160) -> Result<HexBytes, Error> {
        Err(Error::WithKey(format!(
            "account code[{:?}] not found",
            address
        )))
    }

    fn get_miss_usage(&self) -> AvgCounterResult {
        AvgCounterResult::default()
    }

    fn get_storage(&self, address: &SH160, key: &SH256) -> Result<SH256, Error> {
        Err(Error::WithKey(format!(
            "account storage[{:?} {:?}] not found",
            address, key
        )))
    }

    fn prefetch_states(
        &self,
        _list: &[FetchState],
        _with_proof: bool,
    ) -> Result<Vec<FetchStateResult>, Error> {
        unimplemented!()
    }

    fn with_acc(&self, _address: &SH160) -> Self {
        ()
    }
}
