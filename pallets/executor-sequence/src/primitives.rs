#![feature(let_else)]
use frame_support::{pallet_prelude::DispatchError, traits::tokens::Balance};
use t3rn_types::sfx::SideEffect;


pub type ExecutionId = u64;

pub trait ExecutionSequenceTrait<AccountId, Balance> {
     /// Create execution sequence
     fn create_sequence(sender: &AccountId, data: &Vec<u8>) -> Result<(), DispatchError>;
     /// Get execution sequence
     fn get_sequence(execution_id: &ExecutionId) -> Result<Vec<SideEffect<AccountId, Balance>>, DispatchError>;
}