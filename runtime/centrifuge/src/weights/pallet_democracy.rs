//! Autogenerated weights for pallet_democracy
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-04-12, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_democracy
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_democracy.rs
// --template=./scripts/runtime-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use pallet_democracy::weights::WeightInfo;
use sp_std::marker::PhantomData;

/// Weights for pallet_democracy using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn propose() -> Weight {
		(105_365_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn second(s: u32) -> Weight {
		(59_667_000 as Weight) // Standard Error: 6_000
			.saturating_add((358_000 as Weight).saturating_mul(s as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn vote_new(r: u32) -> Weight {
		(73_563_000 as Weight) // Standard Error: 8_000
			.saturating_add((437_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn vote_existing(r: u32) -> Weight {
		(74_330_000 as Weight) // Standard Error: 4_000
			.saturating_add((360_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn emergency_cancel() -> Weight {
		(34_226_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn blacklist(p: u32) -> Weight {
		(93_026_000 as Weight) // Standard Error: 7_000
			.saturating_add((793_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(6 as Weight))
	}
	fn external_propose(v: u32) -> Weight {
		(17_079_000 as Weight) // Standard Error: 0
			.saturating_add((127_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_majority() -> Weight {
		(2_824_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn external_propose_default() -> Weight {
		(2_979_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn fast_track() -> Weight {
		(34_919_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn veto_external(v: u32) -> Weight {
		(37_059_000 as Weight) // Standard Error: 2_000
			.saturating_add((164_000 as Weight).saturating_mul(v as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn cancel_proposal(p: u32) -> Weight {
		(64_092_000 as Weight) // Standard Error: 5_000
			.saturating_add((705_000 as Weight).saturating_mul(p as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn cancel_referendum() -> Weight {
		(21_864_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn cancel_queued(r: u32) -> Weight {
		(40_850_000 as Weight) // Standard Error: 5_000
			.saturating_add((1_363_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn on_initialize_base(r: u32) -> Weight {
		(4_768_000 as Weight) // Standard Error: 9_000
			.saturating_add((7_215_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn on_initialize_base_with_launch_period(r: u32) -> Weight {
		(14_834_000 as Weight) // Standard Error: 8_000
			.saturating_add((7_223_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn delegate(r: u32) -> Weight {
		(68_943_000 as Weight) // Standard Error: 11_000
			.saturating_add((9_407_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(4 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn undelegate(r: u32) -> Weight {
		(34_601_000 as Weight) // Standard Error: 10_000
			.saturating_add((9_368_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().reads((1 as Weight).saturating_mul(r as Weight)))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
			.saturating_add(T::DbWeight::get().writes((1 as Weight).saturating_mul(r as Weight)))
	}
	fn clear_public_proposals() -> Weight {
		(3_549_000 as Weight).saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_preimage(b: u32) -> Weight {
		(51_800_000 as Weight) // Standard Error: 0
			.saturating_add((4_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn note_imminent_preimage(b: u32) -> Weight {
		(35_134_000 as Weight) // Standard Error: 0
			.saturating_add((3_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn reap_preimage(b: u32) -> Weight {
		(52_698_000 as Weight) // Standard Error: 0
			.saturating_add((2_000 as Weight).saturating_mul(b as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	fn unlock_remove(r: u32) -> Weight {
		(48_218_000 as Weight) // Standard Error: 3_000
			.saturating_add((173_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn unlock_set(r: u32) -> Weight {
		(44_675_000 as Weight) // Standard Error: 3_000
			.saturating_add((370_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(3 as Weight))
	}
	fn remove_vote(r: u32) -> Weight {
		(26_654_000 as Weight) // Standard Error: 3_000
			.saturating_add((289_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn remove_other_vote(r: u32) -> Weight {
		(26_584_000 as Weight) // Standard Error: 3_000
			.saturating_add((317_000 as Weight).saturating_mul(r as Weight))
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
}