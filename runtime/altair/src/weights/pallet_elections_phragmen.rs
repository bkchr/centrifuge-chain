
//! Autogenerated weights for `pallet_elections_phragmen`
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
// --pallet=pallet_elections_phragmen
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_elections_phragmen.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_elections_phragmen`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_elections_phragmen::WeightInfo for WeightInfo<T> {
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[1, 5]`.
	fn vote_equal(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `430 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(28_487_207, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 24_782
			.saturating_add(Weight::from_parts(524_123, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_more(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `399 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(40_798_323, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 15_506
			.saturating_add(Weight::from_parts(102_541, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// The range of component `v` is `[2, 5]`.
	fn vote_less(v: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `431 + v * (80 ±0)`
		//  Estimated: `4764 + v * (80 ±0)`
		// Minimum execution time: 40_000_000 picoseconds.
		Weight::from_parts(40_913_737, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			// Standard Error: 22_270
			.saturating_add(Weight::from_parts(114_656, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(Weight::from_parts(0, 80).saturating_mul(v.into()))
	}
	/// Storage: Elections Voting (r:1 w:1)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	fn remove_voter() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `601`
		//  Estimated: `4764`
		// Minimum execution time: 42_000_000 picoseconds.
		Weight::from_parts(43_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4764))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn submit_candidacy(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1508 + c * (48 ±0)`
		//  Estimated: `2993 + c * (48 ±0)`
		// Minimum execution time: 30_000_000 picoseconds.
		Weight::from_parts(33_858_082, 0)
			.saturating_add(Weight::from_parts(0, 2993))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	fn renounce_candidacy_candidate(c: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `351 + c * (48 ±0)`
		//  Estimated: `1836 + c * (48 ±0)`
		// Minimum execution time: 26_000_000 picoseconds.
		Weight::from_parts(26_855_488, 0)
			.saturating_add(Weight::from_parts(0, 1836))
			// Standard Error: 10_984
			.saturating_add(Weight::from_parts(177_096, 0).saturating_mul(c.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 48).saturating_mul(c.into()))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_members() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1688`
		//  Estimated: `3173`
		// Minimum execution time: 39_000_000 picoseconds.
		Weight::from_parts(44_000_000, 0)
			.saturating_add(Weight::from_parts(0, 3173))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	fn renounce_candidacy_runners_up() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1056`
		//  Estimated: `2541`
		// Minimum execution time: 28_000_000 picoseconds.
		Weight::from_parts(34_000_000, 0)
			.saturating_add(Weight::from_parts(0, 2541))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: Benchmark Override (r:0 w:0)
	/// Proof Skipped: Benchmark Override (max_values: None, max_size: None, mode: Measured)
	fn remove_member_without_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 500_000_000_000 picoseconds.
		Weight::from_parts(500_000_000_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
	}
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:2 w:2)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:1 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	fn remove_member_with_replacement() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1791`
		//  Estimated: `6196`
		// Minimum execution time: 64_000_000 picoseconds.
		Weight::from_parts(66_000_000, 0)
			.saturating_add(Weight::from_parts(0, 6196))
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(6))
	}
	/// Storage: Elections Voting (r:101 w:100)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:0)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:0)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Candidates (r:1 w:0)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Balances Locks (r:100 w:100)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:100 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(209), added: 2684, mode: MaxEncodedLen)
	/// Storage: System Account (r:100 w:100)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `v` is `[50, 100]`.
	/// The range of component `d` is `[0, 50]`.
	fn clean_defunct_voters(v: u32, d: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1226 + v * (457 ±0)`
		//  Estimated: `4528 + d * (1 ±0) + v * (3774 ±0)`
		// Minimum execution time: 2_509_000_000 picoseconds.
		Weight::from_parts(2_516_000_000, 0)
			.saturating_add(Weight::from_parts(0, 4528))
			// Standard Error: 205_592
			.saturating_add(Weight::from_parts(28_456_710, 0).saturating_mul(v.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().reads((4_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes((3_u64).saturating_mul(v.into())))
			.saturating_add(Weight::from_parts(0, 1).saturating_mul(d.into()))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(v.into()))
	}
	/// Storage: Elections Candidates (r:1 w:1)
	/// Proof Skipped: Elections Candidates (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Members (r:1 w:1)
	/// Proof Skipped: Elections Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections RunnersUp (r:1 w:1)
	/// Proof Skipped: Elections RunnersUp (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Elections Voting (r:101 w:0)
	/// Proof Skipped: Elections Voting (max_values: None, max_size: None, mode: Measured)
	/// Storage: Council Proposals (r:1 w:0)
	/// Proof Skipped: Council Proposals (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: System Account (r:3 w:3)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Elections ElectionRounds (r:1 w:1)
	/// Proof Skipped: Elections ElectionRounds (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Members (r:0 w:1)
	/// Proof Skipped: Council Members (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Council Prime (r:0 w:1)
	/// Proof Skipped: Council Prime (max_values: Some(1), max_size: None, mode: Measured)
	/// The range of component `c` is `[1, 20]`.
	/// The range of component `v` is `[1, 100]`.
	/// The range of component `e` is `[100, 500]`.
	fn election_phragmen(c: u32, v: u32, e: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + e * (23 ±0) + v * (241 ±0)`
		//  Estimated: `9384 + c * (154 ±35) + e * (19 ±0) + v * (2526 ±2)`
		// Minimum execution time: 212_000_000 picoseconds.
		Weight::from_parts(213_000_000, 0)
			.saturating_add(Weight::from_parts(0, 9384))
			// Standard Error: 482_403
			.saturating_add(Weight::from_parts(2_612_303, 0).saturating_mul(c.into()))
			// Standard Error: 95_963
			.saturating_add(Weight::from_parts(5_084_898, 0).saturating_mul(v.into()))
			// Standard Error: 20_840
			.saturating_add(Weight::from_parts(143_224, 0).saturating_mul(e.into()))
			.saturating_add(T::DbWeight::get().reads(17))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(7))
			.saturating_add(Weight::from_parts(0, 154).saturating_mul(c.into()))
			.saturating_add(Weight::from_parts(0, 19).saturating_mul(e.into()))
			.saturating_add(Weight::from_parts(0, 2526).saturating_mul(v.into()))
	}
}