

use frame_support::{pallet_prelude::DispatchError, traits::tokens::Balance};
use codec::{Encode, Decode};
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;


pub type ExecutionId = u64;
pub type TargetId = [u8; 4];
pub type Sfx4bId = [u8; 4];
pub type Bytes = Vec<u8>;

pub const MULTI_TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"mult";
pub const TRANSFER_SIDE_EFFECT_ID: &[u8; 4] = b"tran";
pub const SWAP_SIDE_EFFECT_ID: &[u8; 4] = b"swap";
pub const ERROR_SIDE_EFFECT_ID: &[u8; 4] = b"errr";


#[derive(Clone, Eq, PartialEq, Encode, Decode, Debug, TypeInfo)]
pub struct SideEffect<AccountId, BalanceOf> {
    pub target: TargetId,
    pub max_reward: BalanceOf,
    pub insurance: BalanceOf,
    pub action: Sfx4bId,
    pub encoded_args: Vec<Bytes>,
    pub signature: Bytes,
    pub enforce_executor: Option<AccountId>,
    pub reward_asset_id: Option<u32>,
}


pub enum Action {
     Transfer,
     TransferMulti,
     Swap,
     None,
 }
 
 impl TryFrom<u8> for Action {
     type Error = &'static str;
 
     fn try_from(value: u8) -> Result<Self, Self::Error> {
         match value {
             0 => Ok(Action::Transfer),
             1 => Ok(Action::TransferMulti),
             2 => Ok(Action::Swap),
             _ => Err("Invalid action id"),
         }
     }
 }
 
 impl From<Action> for [u8; 4] {
     fn from(val: Action) -> Self {
         match val {
             Action::Transfer => *TRANSFER_SIDE_EFFECT_ID,
             Action::Swap => *SWAP_SIDE_EFFECT_ID,
             Action::TransferMulti => *MULTI_TRANSFER_SIDE_EFFECT_ID,
             Action::None => *ERROR_SIDE_EFFECT_ID,
         }
     }
 }

 pub trait ExecutionSequenceTrait<AccountId, Balance> {
     /// Create execution sequence
     fn create_sequence(sender: &AccountId, data: &Vec<u8>) -> Result<(), DispatchError>;
     /// Get execution sequence
     fn get_sequence(execution_id: &ExecutionId) -> Result<Vec<SideEffect<AccountId, Balance>>, DispatchError>;
}
