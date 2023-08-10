#![cfg(test)]

use frame_support::{
                    parameter_types,
                    traits::ConstU32,
                    PalletId,
};
use sp_core::H256;
use sp_runtime::{
    generic,
    traits::{BlakeTwo256, IdentityLookup, Keccak256, AccountIdConversion},
};
use sp_std::{boxed::Box, prelude::*};
use crate as pallet_executor_sequence;

//use crate::pallet::Config;
use super::*;

pub type AccountId = u128;
pub type Balance = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;


parameter_types! {
                    pub const BlockHashCount: u32 = 250;
                    pub BlockWeights: frame_system::limits::BlockWeights =
                        frame_system::limits::BlockWeights::simple_max(1024);
}
impl frame_system::Config for Test {
                    type AccountData = pallet_balances::AccountData<u64>;
                    type AccountId = u128;
                    type BaseCallFilter = frame_support::traits::Everything;
                    type BlockHashCount = BlockHashCount;
                    type BlockLength = ();
                    type BlockNumber = u32;
                    type BlockWeights = ();
                    type Call = Call;
                    type DbWeight = ();
                    type Event = Event;
                    type Hash = H256;
                    type Hashing = Keccak256;
                    type Header = generic::Header<u32, BlakeTwo256>;
                    type Index = u64;
                    type Lookup = IdentityLookup<Self::AccountId>;
                    type MaxConsumers = ConstU32<16>;
                    type OnKilledAccount = ();
                    type OnNewAccount = ();
                    type OnSetCode = ();
                    type Origin = Origin;
                    type PalletInfo = PalletInfo;
                    type SS58Prefix = ();
                    type SystemWeightInfo = ();
                    type Version = ();
}

parameter_types! {
                    pub const ExistentialDeposit: u64 = 1;
}

impl pallet_balances::Config for Test {
                    type AccountStore = System;
                    type Balance = u64;
                    type DustRemoval = ();
                    type Event = Event;
                    type ExistentialDeposit = ExistentialDeposit;
                    type MaxLocks = ();
                    type MaxReserves = ();
                    type ReserveIdentifier = ();
                    type WeightInfo = ();
}

parameter_types! {
                    //pub const StorageFee: Balance = 1;
                    pub const MaxSideEffectPerSequence: u64 = 100;
                   // pub const TreasuryPalletId: PalletId = PalletId(*b"t3rn/trs");
	                //pub const TreasuryModuleAccount: AccountId = TreasuryPalletId::get().into_account_truncating();
}

impl pallet_executor_sequence::Config for Test {
    type Event = Event;
    type Currency = Balances;
    type MaxSideEffectPerSequence = MaxSideEffectPerSequence;
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
                    pub enum Test where
                        Block = Block,
                        NodeBlock = Block,
                        UncheckedExtrinsic = UncheckedExtrinsic
                    {
                                        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
                                        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
                                        Sequence: pallet_executor_sequence::{Pallet, Call, Storage, Event<T>},
                    }
);

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::default().build_storage::<Test>().unwrap().into()
}

