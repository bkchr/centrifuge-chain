
//! Autogenerated weights for `pallet_multisig`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-06-07, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner`, CPU: `AMD EPYC 7763 64-Core Processor`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("altair-local")`, DB CACHE: 1024

// Executed Command:
// target/release/centrifuge-chain
// benchmark
// pallet
// --chain=altair-local
// --steps=50
// --repeat=20
// --pallet=pallet_multisig
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_multisig.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_multisig`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_multisig::WeightInfo for WeightInfo<T> {
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_threshold_1(z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 15_920_000 picoseconds.
		Weight::from_parts(16_791_683, 0)
			.saturating_add(Weight::from_parts(0, 0))
			// Standard Error: 5
			.saturating_add(Weight::from_parts(494, 0).saturating_mul(z.into()))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_create(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 50_905_000 picoseconds.
		Weight::from_parts(43_938_808, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 750
			.saturating_add(Weight::from_parts(90_272, 0).saturating_mul(s.into()))
			// Standard Error: 7
			.saturating_add(Weight::from_parts(1_537, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[3, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_approve(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 31_800_000 picoseconds.
		Weight::from_parts(24_763_259, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 637
			.saturating_add(Weight::from_parts(84_167, 0).saturating_mul(s.into()))
			// Standard Error: 6
			.saturating_add(Weight::from_parts(1_526, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	/// The range of component `z` is `[0, 10000]`.
	fn as_multi_complete(s: u32, z: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `456 + s * (33 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 58_639_000 picoseconds.
		Weight::from_parts(47_679_049, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 1_118
			.saturating_add(Weight::from_parts(126_990, 0).saturating_mul(s.into()))
			// Standard Error: 10
			.saturating_add(Weight::from_parts(1_605, 0).saturating_mul(z.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_create(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `334 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 40_445_000 picoseconds.
		Weight::from_parts(41_735_038, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 1_330
			.saturating_add(Weight::from_parts(95_912, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn approve_as_multi_approve(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `315`
		//  Estimated: `6811`
		// Minimum execution time: 21_761_000 picoseconds.
		Weight::from_parts(22_759_304, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 690
			.saturating_add(Weight::from_parts(84_536, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Multisig::Multisigs` (r:1 w:1)
	/// Proof: `Multisig::Multisigs` (`max_values`: None, `max_size`: Some(3346), added: 5821, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[2, 100]`.
	fn cancel_as_multi(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `520 + s * (1 ±0)`
		//  Estimated: `6811`
		// Minimum execution time: 41_448_000 picoseconds.
		Weight::from_parts(42_609_737, 0)
			.saturating_add(Weight::from_parts(0, 6811))
			// Standard Error: 774
			.saturating_add(Weight::from_parts(89_590, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
