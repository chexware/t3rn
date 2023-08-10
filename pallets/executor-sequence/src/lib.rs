#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
//pub use pallet::*;
use sp_runtime::ArithmeticError;
use sp_std::{convert::TryInto, vec, vec::Vec};
use frame_system::{
                    ensure_signed,
                    pallet_prelude::*, 
};
use frame_support::{
	dispatch::DispatchResultWithPostInfo,
                    traits::{Currency, Get, ExistenceRequirement},
                    pallet_prelude::*, PalletId,
};

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod primitives;

use primitives::*;
use crate::pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::pallet]
                    #[pallet::without_storage_info]
	pub struct Pallet<T>(_);
                    
                    pub type BalanceOf<T> = <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

                                        /// The maximum amount of side effects
		#[pallet::constant]
		type MaxSideEffectPerSequence: Get<u64>;

                                        /// Currency type for handling storage fees, and other fungible asset-related activities
                                        type Currency: Currency<Self::AccountId>;
	}

	/// Tracks the next execution ID
                    #[pallet::storage]
                    #[pallet::getter(fn next_execution_id)]
                    pub type NextExecutionId<T: Config> = StorageValue<_, ExecutionId, ValueQuery>;

                    /// Executions store a map of ExecutionStep-s organised by ExecutionId, generated through the create_execution_sequence extrinsic
                    #[pallet::storage]
                    #[pallet::getter(fn get_execution_sequence)]
                    pub type Executions<T: Config> = StorageMap<_, Twox64Concat, ExecutionId, ExecutionStep<T::AccountId,BalanceOf<T>>, OptionQuery>;

                    #[pallet::event]
                    #[pallet::generate_deposit(pub (super) fn deposit_event)]
                    pub enum Event<T: Config> {
                                        /// A new execution was created
                                        ExecutionSequenceCreated(ExecutionId),
                    }
                    
	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Invalid data for execution sequence
		InvalidData,
	}

	// Dispatchable functions allows users to interact with the pallet and invoke state changes.
	// These functions materialize as "extrinsics", which are often compared to transactions.
	// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		// Create execution sequence of SideEffects with ExecutionId from encoded data.
		///
		/// The dispatch origin for this call must be _Signed_.
		/// `data`: the encoded data as vector of bytes
		///
		/// Emits `ExecutionSequenceCreated` if successful.
		#[pallet::weight(Weight::from_parts(195_000, 1) + T::DbWeight::get().writes(1))]
                                        pub fn create_execution_sequence(
                                                            origin: OriginFor<T>,
                                                            data: Vec<u8>,
                                        ) -> DispatchResultWithPostInfo {
                                                            let origin = ensure_signed(origin)?;
                                                            
                                                            Self::do_create_execution_sequence(&origin, &data)
                                        }
	}

	impl<T: Config> Pallet<T> {
                                        /// Internal creation of execution sequence
                                        pub fn do_create_execution_sequence(creator: &T::AccountId, data: &Vec<u8>) -> DispatchResultWithPostInfo  {
                                                            let mut side_effects_sequence: Vec<SideEffect<T::AccountId, BalanceOf<T>>> = Vec::new();
                    
                                                            // Read data
                                                            for side_effect_data in data.splitn(T::MaxSideEffectPerSequence::get().try_into().unwrap(), |s| *s == 59 ) { // 59 - ";"  as byte
                                                                                let side_effect = Self::create_side_effect(side_effect_data.to_vec())?;
                                                                                side_effects_sequence.push(side_effect);
                                                            } 

                                                            // Create execution step   
                                                            let execution_step: ExecutionStep<T::AccountId, BalanceOf<T>> = ExecutionStep {
                                                                                side_effects: side_effects_sequence,
                                                                                status: ExecutionStatus::Unprocessed,
                                                            };                                       
                    
                                                            // Pay storage fee for populating the Executions storage map
                                                            //T::Currency::transfer(creator, &T::NetworkTreasuryAccount::get(), T::StorageFee::get(), ExistenceRequirement::KeepAlive)?;
                    
                                                            // Store side effect
                                                            let execution_id = Self::next_execution_id();
                                                            let next_execution_id = execution_id.checked_add(1).ok_or(ArithmeticError::Overflow)?;
                                                            <NextExecutionId<T>>::put(next_execution_id.clone());
                                                            <Executions<T>>::insert(execution_id, execution_step);
                                       
                                                            Self::deposit_event(Event::<T>::ExecutionSequenceCreated(next_execution_id));
                                                            Ok(().into())
                                        }
                    
                                        /// Internal creation of side effect
                                        fn create_side_effect(side_effect_data: Vec<u8>) -> Result<SideEffect<T::AccountId, BalanceOf<T>>, DispatchError> {
                                                            let mut target_network: TargetId = [0u8; 4];
                                                            let mut data = side_effect_data.clone();
                                                            // Decode network byte
                                        
                                                            match  data.remove(0) {
                                                                                107 => { // "k" - Kusama
                                                                                                    target_network = *b"ksma";
                                                                                }
                                                                                112 => { // "p" - Polkadot 
                                                                                                    target_network = *b"pdot";
                                                                                }
                                                                                114 => { // "r" - Rococo
                                                                                                    target_network = *b"roco";
                                                                                }
                                                                                116 => { // "t" - T3rn
                                                                                                    target_network = *b"t3rn";
                                                                                }
                                                                                _=> return Err(Error::<T>::InvalidData.into()),
                                                            }
                    
                                                            // Decode action byte
                                                            let mut target_action: Action = Action::None;
                                                            match data.remove(0) {
                                                                                109 => { // "m" - multi_tran
                                                                                                    target_action = Action::TransferMulti;
                                                                                }
                                                                                115 => { // "s" - swap
                                                                                                    target_action = Action::Swap;
                                                                                }
                                                                                116  => { // "t" - tran
                                                                                                    target_action = Action::Transfer;
                                                                                }
                                                                                _=> return Err(Error::<T>::InvalidData.into()),
                                                            }
                    
                                                            // Decode arguments
                                                            let action_arguments = Self::extract_arguments(&target_action, data)?;
                                                            let mut arguments_vec: Vec<Vec<u8>> = Vec::new();
                                                            arguments_vec.push(action_arguments);

                                                            Ok(SideEffect::<T::AccountId, BalanceOf<T>>{
                                                                                target: target_network,
                                                                                max_reward: Default::default(),
                                                                                action: target_action.into(),
                                                                                encoded_args: arguments_vec, 
                                                                                signature: vec![],
                                                                                insurance: Default::default(),
                                                                                enforce_executor: None,
                                                                                reward_asset_id: None,
                                                            })
                                        }
                    
                                        /// Internal action arguments extran
                                        fn extract_arguments(action: &Action, data: Vec<u8>) -> Result<Vec<u8>, DispatchError> {
                                                            let mut data_iter = data.split(|c|  *c ==  124); // "|" = 124
                    
                                                            match *action {
                                                                                Action::TransferMulti => {
                                                                                                    if data_iter.count() != 5 {
                                                                                                                        return Err(Error::<T>::InvalidData.into());               
                                                                                                    }
                                                                                                    Ok(data)
                                                                                }
                                                                                Action::Transfer => {
                                                                                                    if data_iter.count() != 4 {
                                                                                                                        return Err(Error::<T>::InvalidData.into());               
                                                                                                    }
                                                                                                    Ok(data)
                                                                                }
                                                                                Action::Swap => {
                                                                                                    if data_iter.count() != 6 {
                                                                                                                        return Err(Error::<T>::InvalidData.into());              
                                                                                                    }
                                                                                                    Ok(data)
                                                                                }
                                                                                _=> Err(Error::<T>::InvalidData.into()),
                                                            }
                                        }
                    }
}

impl<T: Config> ExecutionSequenceTrait<T::AccountId, BalanceOf<T>> for Pallet<T>  {

                    fn create_sequence(sender: &T::AccountId, data: &Vec<u8>) -> Result<(), DispatchError> {
                                        Self::do_create_execution_sequence(sender, data);
                                        Ok(())
                    }

                    fn get_sequence(execution_id: &ExecutionId) -> Result<Option<ExecutionStep<T::AccountId, BalanceOf<T>>>, DispatchError> {
                                        let sequence = Self::get_execution_sequence(execution_id);
                                        Ok(sequence)
                    }

                    fn update_execution_status(execution_id: &ExecutionId, new_status: ExecutionStatus) -> Result<(), DispatchError> {
                                        <Executions<T>>::try_mutate_exists(execution_id, |execution_step| -> DispatchResult {
                                                            let mut execution_step = execution_step.as_mut().ok_or(Error::<T>::InvalidData)?;
                                                            execution_step.status = new_status;
                                                            Ok(().into())
                                        })?;
                                        Ok(())
                    }
}
