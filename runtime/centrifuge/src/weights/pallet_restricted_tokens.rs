
//! Autogenerated weights for `pallet_restricted_tokens`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-04-04, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `runner`, CPU: `Intel(R) Xeon(R) Platinum 8272CL CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("centrifuge-dev"), DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=centrifuge-dev
// --steps=50
// --repeat=20
// --pallet=pallet_restricted_tokens
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/centrifuge/src/weights/pallet_restricted_tokens.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_restricted_tokens`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_restricted_tokens::WeightInfo for WeightInfo<T> {
	// Storage: System Account (r:1 w:1)
	fn transfer_native() -> Weight {
		// Minimum execution time: 104_103 nanoseconds.
		Weight::from_ref_time(105_302_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_other() -> Weight {
		// Minimum execution time: 86_902 nanoseconds.
		Weight::from_ref_time(88_002_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive_native() -> Weight {
		// Minimum execution time: 75_102 nanoseconds.
		Weight::from_ref_time(76_202_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_keep_alive_other() -> Weight {
		// Minimum execution time: 80_402 nanoseconds.
		Weight::from_ref_time(81_602_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn transfer_all_native() -> Weight {
		// Minimum execution time: 87_402 nanoseconds.
		Weight::from_ref_time(88_702_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn transfer_all_other() -> Weight {
		// Minimum execution time: 91_302 nanoseconds.
		Weight::from_ref_time(92_402_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn force_transfer_native() -> Weight {
		// Minimum execution time: 83_001 nanoseconds.
		Weight::from_ref_time(84_302_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:2 w:2)
	// Storage: System Account (r:1 w:1)
	fn force_transfer_other() -> Weight {
		// Minimum execution time: 86_602 nanoseconds.
		Weight::from_ref_time(87_702_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: System Account (r:1 w:1)
	fn set_balance_native() -> Weight {
		// Minimum execution time: 86_302 nanoseconds.
		Weight::from_ref_time(87_302_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: OrmlTokens Accounts (r:1 w:1)
	// Storage: OrmlTokens TotalIssuance (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	fn set_balance_other() -> Weight {
		// Minimum execution time: 98_302 nanoseconds.
		Weight::from_ref_time(99_503_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
}