
//! Autogenerated weights for `pallet_pool_registry`
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
// --pallet=pallet_pool_registry
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_pool_registry.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_pool_registry`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_pool_registry::WeightInfo for WeightInfo<T> {
	/// Storage: `PoolRegistry::Pools` (r:1 w:1)
	/// Proof: `PoolRegistry::Pools` (`max_values`: None, `max_size`: Some(25), added: 2500, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:6 w:5)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::AccountDeposit` (r:1 w:1)
	/// Proof: `PoolSystem::AccountDeposit` (`max_values`: None, `max_size`: Some(64), added: 2539, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::LastFeeId` (r:1 w:1)
	/// Proof: `PoolFees::LastFeeId` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::FeeIdsToPoolBucket` (r:100 w:100)
	/// Proof: `PoolFees::FeeIdsToPoolBucket` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::FeeIds` (r:1 w:1)
	/// Proof: `PoolFees::FeeIds` (`max_values`: None, `max_size`: Some(843), added: 3318, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:1)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `Permissions::PermissionCount` (r:1 w:1)
	/// Proof: `Permissions::PermissionCount` (`max_values`: None, `max_size`: Some(46), added: 2521, mode: `MaxEncodedLen`)
	/// Storage: `Permissions::Permission` (r:1 w:1)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `Loans::WriteOffPolicy` (r:0 w:1)
	/// Proof: `Loans::WriteOffPolicy` (`max_values`: None, `max_size`: Some(5126), added: 7601, mode: `MaxEncodedLen`)
	/// Storage: `PoolRegistry::PoolMetadata` (r:0 w:1)
	/// Proof: `PoolRegistry::PoolMetadata` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::PoolDeposit` (r:0 w:1)
	/// Proof: `PoolSystem::PoolDeposit` (`max_values`: None, `max_size`: Some(72), added: 2547, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn register(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `422`
		//  Estimated: `17508 + m * (2508 ±0) + n * (3417 ±0)`
		// Minimum execution time: 186_748_000 picoseconds.
		Weight::from_parts(123_689_120, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 81_503
			.saturating_add(Weight::from_parts(24_132_712, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(10))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(m.into())))
			.saturating_add(T::DbWeight::get().writes(11))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(m.into())))
			.saturating_add(Weight::from_parts(0, 2508).saturating_mul(m.into()))
			.saturating_add(Weight::from_parts(0, 3417).saturating_mul(n.into()))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:0)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:0)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:0)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:0)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::ScheduledUpdate` (r:0 w:1)
	/// Proof: `PoolSystem::ScheduledUpdate` (`max_values`: None, `max_size`: Some(1504), added: 3979, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn update_no_execution(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `924 + m * (124 ±0) + n * (133 ±0)`
		//  Estimated: `17508 + n * (2531 ±0)`
		// Minimum execution time: 58_940_000 picoseconds.
		Weight::from_parts(50_984_763, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 38_828
			.saturating_add(Weight::from_parts(2_943_073, 0).saturating_mul(n.into()))
			// Standard Error: 1_771
			.saturating_add(Weight::from_parts(217_699, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(Weight::from_parts(0, 2531).saturating_mul(n.into()))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:0)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:0)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:5 w:1)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:0)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::ScheduledUpdate` (r:0 w:1)
	/// Proof: `PoolSystem::ScheduledUpdate` (`max_values`: None, `max_size`: Some(1504), added: 3979, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn update_and_execute(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `927 + m * (124 ±0) + n * (200 ±0)`
		//  Estimated: `17508 + n * (3417 ±0)`
		// Minimum execution time: 101_439_000 picoseconds.
		Weight::from_parts(70_893_079, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 47_685
			.saturating_add(Weight::from_parts(10_729_260, 0).saturating_mul(n.into()))
			// Standard Error: 2_176
			.saturating_add(Weight::from_parts(229_716, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 3417).saturating_mul(n.into()))
	}
	/// Storage: `PoolSystem::Pool` (r:1 w:1)
	/// Proof: `PoolSystem::Pool` (`max_values`: None, `max_size`: Some(813), added: 3288, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::EpochExecution` (r:1 w:0)
	/// Proof: `PoolSystem::EpochExecution` (`max_values`: None, `max_size`: Some(754), added: 3229, mode: `MaxEncodedLen`)
	/// Storage: `PoolSystem::ScheduledUpdate` (r:1 w:1)
	/// Proof: `PoolSystem::ScheduledUpdate` (`max_values`: None, `max_size`: Some(1504), added: 3979, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Investments::ActiveRedeemOrders` (r:5 w:0)
	/// Proof: `Investments::ActiveRedeemOrders` (`max_values`: None, `max_size`: Some(56), added: 2531, mode: `MaxEncodedLen`)
	/// Storage: `OrmlAssetRegistry::Metadata` (r:5 w:1)
	/// Proof: `OrmlAssetRegistry::Metadata` (`max_values`: None, `max_size`: Some(942), added: 3417, mode: `MaxEncodedLen`)
	/// Storage: `PoolFees::ActiveFees` (r:1 w:0)
	/// Proof: `PoolFees::ActiveFees` (`max_values`: None, `max_size`: Some(14043), added: 16518, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[1, 5]`.
	/// The range of component `m` is `[0, 100]`.
	fn execute_update(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `915 + m * (124 ±0) + n * (227 ±0)`
		//  Estimated: `17508 + n * (3417 ±0)`
		// Minimum execution time: 94_327_000 picoseconds.
		Weight::from_parts(61_924_136, 0)
			.saturating_add(Weight::from_parts(0, 17508))
			// Standard Error: 60_907
			.saturating_add(Weight::from_parts(11_430_089, 0).saturating_mul(n.into()))
			// Standard Error: 2_779
			.saturating_add(Weight::from_parts(226_144, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().reads((2_u64).saturating_mul(n.into())))
			.saturating_add(T::DbWeight::get().writes(3))
			.saturating_add(Weight::from_parts(0, 3417).saturating_mul(n.into()))
	}
	/// Storage: `Permissions::Permission` (r:1 w:0)
	/// Proof: `Permissions::Permission` (`max_values`: None, `max_size`: Some(228), added: 2703, mode: `MaxEncodedLen`)
	/// Storage: `PoolRegistry::PoolMetadata` (r:0 w:1)
	/// Proof: `PoolRegistry::PoolMetadata` (`max_values`: None, `max_size`: Some(71), added: 2546, mode: `MaxEncodedLen`)
	/// The range of component `n` is `[0, 46]`.
	/// The range of component `m` is `[0, 100]`.
	fn set_metadata(n: u32, m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `146`
		//  Estimated: `3693`
		// Minimum execution time: 19_066_000 picoseconds.
		Weight::from_parts(19_700_503, 0)
			.saturating_add(Weight::from_parts(0, 3693))
			// Standard Error: 1_506
			.saturating_add(Weight::from_parts(7_088, 0).saturating_mul(n.into()))
			// Standard Error: 700
			.saturating_add(Weight::from_parts(30_038, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
