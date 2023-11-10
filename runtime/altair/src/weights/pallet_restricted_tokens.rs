
//! Autogenerated weights for `pallet_restricted_tokens`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-10, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `kf-FG`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("altair-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-dev
// --steps=50
// --repeat=20
// --pallet=pallet_restricted_tokens
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_restricted_tokens.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_restricted_tokens`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_restricted_tokens::WeightInfo for WeightInfo<T> {
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3593`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(59_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `644`
		//  Estimated: `6198`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(43_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3593`
		// Minimum execution time: 49_000_000 picoseconds.
		Weight::from_parts(50_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_keep_alive_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `541`
		//  Estimated: `6198`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(40_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3593`
		// Minimum execution time: 61_000_000 picoseconds.
		Weight::from_parts(61_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn transfer_all_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `644`
		//  Estimated: `6198`
		// Minimum execution time: 46_000_000 picoseconds.
		Weight::from_parts(46_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `226`
		//  Estimated: `3593`
		// Minimum execution time: 58_000_000 picoseconds.
		Weight::from_parts(60_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: OrmlTokens Accounts (r:2 w:2)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn force_transfer_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `644`
		//  Estimated: `6198`
		// Minimum execution time: 43_000_000 picoseconds.
		Weight::from_parts(43_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6198))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: Balances Holds (r:1 w:1)
	/// Proof: Balances Holds (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:1)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn set_balance_native() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `265`
		//  Estimated: `3674`
		// Minimum execution time: 118_000_000 picoseconds.
		Weight::from_parts(120_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3674))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: OrmlTokens Accounts (r:1 w:1)
	/// Proof: OrmlTokens Accounts (max_values: None, max_size: Some(129), added: 2604, mode: MaxEncodedLen)
	/// Storage: OrmlAssetRegistry Metadata (r:1 w:0)
	/// Proof Skipped: OrmlAssetRegistry Metadata (max_values: None, max_size: None, mode: Measured)
	/// Storage: OrmlTokens TotalIssuance (r:1 w:1)
	/// Proof: OrmlTokens TotalIssuance (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	fn set_balance_other() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `467`
		//  Estimated: `3932`
		// Minimum execution time: 68_000_000 picoseconds.
		Weight::from_parts(69_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3932))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
}
