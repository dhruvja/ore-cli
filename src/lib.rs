pub mod args;
pub mod balance;
pub mod benchmark;
pub mod busses;
pub mod claim;
pub mod close;
pub mod config;
pub mod cu_limits;
pub mod dynamic_fee;
#[cfg(feature = "admin")]
pub mod initialize;
pub mod mine;
pub mod open;
pub mod proof;
pub mod rewards;
pub mod send_and_confirm;
pub mod stake;
pub mod transfer;
pub mod upgrade;
pub mod utils;

use std::sync::Arc;

use args::*;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::signature::{read_keypair_file, Keypair};

pub struct Miner {
  pub keypair_filepath: Option<String>,
  pub priority_fee: Option<u64>,
  pub dynamic_fee_url: Option<String>,
  pub dynamic_fee: bool,
  pub rpc_client: Arc<RpcClient>,
  pub fee_payer_filepath: Option<String>,
  pub jito_client: Arc<RpcClient>,
  pub tip: Arc<std::sync::RwLock<u64>>,
}

impl Miner {
  pub fn new(
      rpc_client: Arc<RpcClient>,
      priority_fee: Option<u64>,
      keypair_filepath: Option<String>,
      dynamic_fee_url: Option<String>,
      dynamic_fee: bool,
      fee_payer_filepath: Option<String>,
      jito_client: Arc<RpcClient>,
      tip: Arc<std::sync::RwLock<u64>>,
  ) -> Self {
      Self {
          rpc_client,
          keypair_filepath,
          priority_fee,
          dynamic_fee_url,
          dynamic_fee,
          fee_payer_filepath,
          jito_client,
          tip,
      }
  }

  pub fn signer(&self) -> Keypair {
      match self.keypair_filepath.clone() {
          Some(filepath) => read_keypair_file(filepath.clone())
              .expect(format!("No keypair found at {}", filepath).as_str()),
          None => panic!("No keypair provided"),
      }
  }

  pub fn fee_payer(&self) -> Keypair {
      match self.fee_payer_filepath.clone() {
          Some(filepath) => read_keypair_file(filepath.clone())
              .expect(format!("No fee payer keypair found at {}", filepath).as_str()),
          None => panic!("No fee payer keypair provided"),
      }
  }
}