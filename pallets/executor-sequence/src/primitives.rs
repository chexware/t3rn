use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use frame_support::{pallet_prelude::{DispatchError, DispatchResult}, traits::tokens::Balance};
use t3rn_types::sfx::SideEffect;


pub type ExecutionId = u64;

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebug, TypeInfo)]
pub enum ExecutionStatus {
    Confirmed,
    Reverted,
    Unprocessed,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct ExecutionStep<AccountId,Balance> {
    pub side_effects: Vec<SideEffect<AccountId,Balance>>,
    pub status: ExecutionStatus,
}


pub trait ExecutionSequenceTrait<AccountId, Balance> {
     /// Create execution sequence
     fn create_sequence(sender: &AccountId, data: &Vec<u8>) -> Result<(), DispatchError>;
     /// Get execution sequence
     fn get_sequence(execution_id: &ExecutionId) -> Result<Option<ExecutionStep<AccountId, Balance>>, DispatchError>;
     /// Update execution status
     fn update_execution_status(execution_id: &ExecutionId, new_status: ExecutionStatus) -> Result<(), DispatchError>;
}