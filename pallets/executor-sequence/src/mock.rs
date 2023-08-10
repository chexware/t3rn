
#![cfg(test)]

use frame_support::{
                    construct_runtime, parameter_types,
                    traits::{ConstU16, ConstU32, ConstU64, ConstU128},
                    PalletId,
};
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup, Keccak256, AccountIdConversion},
	BuildStorage,
};
use sp_std::{boxed::Box, prelude::*};

use crate as pallet_executor_sequence;

use super::*;

pub type AccountId = u64;
pub type Balance = u64;

pub const ALICE: AccountId = 1;
pub const BOB: AccountId = 2;

type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test
	{
		System: frame_system,
		Balances: pallet_balances,
		Sequence: pallet_executor_sequence,
	}
);


impl frame_system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = ConstU64<250>;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = pallet_balances::AccountData<u64>;
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = ConstU16<42>;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
                    pub const ExistentialDeposit: Balance = 1;
}

impl pallet_balances::Config for Test {
                    type MaxLocks = ConstU32<10>;
	type MaxReserves = ();
	type ReserveIdentifier = [u8; 8];
	/// The type for recording an account's balance.
	type Balance = Balance;
	/// The ubiquitous event type.
	type RuntimeEvent = RuntimeEvent;
	type DustRemoval = ();
	type ExistentialDeposit = ExistentialDeposit;
	type AccountStore = System;
	type WeightInfo = ();
	type FreezeIdentifier = ();
	type MaxFreezes = ();
	type RuntimeHoldReason = ();
	type MaxHolds = ();
}

parameter_types! {
                    pub const StorageFee: Balance = 1;
                    pub const MaxSideEffectPerSequence: u64 = 100;
}

impl pallet_executor_sequence::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type Currency = Balances;
	type MaxSideEffectPerSequence = MaxSideEffectPerSequence;	
}



// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	frame_system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
