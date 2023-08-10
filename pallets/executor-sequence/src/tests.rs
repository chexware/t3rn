use frame_support::assert_ok;
use t3rn_types::sfx::{Action, SideEffect};
use crate::mock::*;



#[test]
fn creating_sequence_with_single_side_effect() {
	new_test_ext().execute_with(|| {
                                        let test_data: Vec<u8> = "pt|x|y|100".as_bytes().to_vec();
                                        assert_ok!(Sequence::create_execution_sequence(Origin::signed(1), test_data));
		let side_effect_1: SideEffect<AccountId,Balance> = SideEffect {
			target: *b"pdot",
			max_reward: Default::default(),
			insurance: Default::default(),
			signature: vec![],
			enforce_executor: None,
			reward_asset_id: None,
			action: Action::Transfer.into(),
			encoded_args: vec!["|x|y|100".as_bytes().to_vec()]

		};
		assert_eq!(Sequence::get_execution_sequence(0),vec![side_effect_1])
	});
}

#[test]
fn creating_sequence_with_single_multiple_side_effects() {
	new_test_ext().execute_with(|| {
		let test_data: Vec<u8> = "pt|x|y|100;tt|x|y|200".as_bytes().to_vec();
                                        assert_ok!(Sequence::create_execution_sequence(Origin::signed(1), test_data));
		let side_effect_1: SideEffect<AccountId,Balance> = SideEffect {
			target: *b"pdot",
			max_reward: Default::default(),
			insurance: Default::default(),
			signature: vec![],
			enforce_executor: None,
			reward_asset_id: None,
			action: Action::Transfer.into(),
			encoded_args: vec!["|x|y|100".as_bytes().to_vec()]

		};
		let side_effect_2: SideEffect<AccountId,Balance> = SideEffect {
			target: *b"t3rn",
			max_reward: Default::default(),
			insurance: Default::default(),
			signature: vec![],
			enforce_executor: None,
			reward_asset_id: None,
			action: Action::Transfer.into(),
			encoded_args: vec!["|x|y|200".as_bytes().to_vec()]

		};
		assert_eq!(Sequence::get_execution_sequence(0),vec![side_effect_1, side_effect_2])
		
	});
}