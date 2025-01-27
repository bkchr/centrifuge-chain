
//! Autogenerated weights for `pallet_identity`
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
// --pallet=pallet_identity
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --output=/tmp/runtime/altair/src/weights/pallet_identity.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_identity`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_identity::WeightInfo for WeightInfo<T> {
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn add_registrar(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `31 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 11_071_000 picoseconds.
		Weight::from_parts(11_956_420, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 2_013
			.saturating_add(Weight::from_parts(113_475, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn set_identity(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6977 + r * (5 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 185_687_000 picoseconds.
		Weight::from_parts(190_458_359, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 9_985
			.saturating_add(Weight::from_parts(118_924, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:100 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 100]`.
	fn set_subs_new(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `100`
		//  Estimated: `11037 + s * (2589 ±0)`
		// Minimum execution time: 13_476_000 picoseconds.
		Weight::from_parts(31_626_105, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 5_424
			.saturating_add(Weight::from_parts(5_141_499, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
			.saturating_add(Weight::from_parts(0, 2589).saturating_mul(s.into()))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `p` is `[0, 100]`.
	fn set_subs_old(p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `193 + p * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 13_575_000 picoseconds.
		Weight::from_parts(30_421_544, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 4_675
			.saturating_add(Weight::from_parts(2_072_423, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn clear_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7069 + r * (5 ±0) + s * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 81_903_000 picoseconds.
		Weight::from_parts(83_404_495, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 13_949
			.saturating_add(Weight::from_parts(100_282, 0).saturating_mul(r.into()))
			// Standard Error: 2_721
			.saturating_add(Weight::from_parts(2_046_604, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn request_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6967 + r * (57 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 121_377_000 picoseconds.
		Weight::from_parts(124_676_842, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 6_660
			.saturating_add(Weight::from_parts(92_087, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	fn cancel_request(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6998`
		//  Estimated: `11037`
		// Minimum execution time: 118_111_000 picoseconds.
		Weight::from_parts(121_516_033, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 6_943
			.saturating_add(Weight::from_parts(53_399, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fee(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_255_000 picoseconds.
		Weight::from_parts(8_828_967, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_075
			.saturating_add(Weight::from_parts(74_131, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_account_id(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_475_000 picoseconds.
		Weight::from_parts(8_900_103, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 950
			.saturating_add(Weight::from_parts(73_969, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:1)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn set_fields(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `88 + r * (57 ±0)`
		//  Estimated: `2626`
		// Minimum execution time: 8_286_000 picoseconds.
		Weight::from_parts(8_679_161, 0)
			.saturating_add(Weight::from_parts(0, 2626))
			// Standard Error: 1_203
			.saturating_add(Weight::from_parts(73_566, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::Registrars` (r:1 w:0)
	/// Proof: `Identity::Registrars` (`max_values`: Some(1), `max_size`: Some(1141), added: 1636, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 19]`.
	fn provide_judgement(r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7045 + r * (57 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 157_844_000 picoseconds.
		Weight::from_parts(163_504_852, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 8_336
			.saturating_add(Weight::from_parts(111_210, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:2 w:2)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:0 w:100)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `s` is `[0, 100]`.
	fn kill_identity(r: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `7396 + r * (15 ±0) + s * (32 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 103_394_000 picoseconds.
		Weight::from_parts(103_065_236, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 14_066
			.saturating_add(Weight::from_parts(192_397, 0).saturating_mul(r.into()))
			// Standard Error: 2_744
			.saturating_add(Weight::from_parts(2_071_673, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(4))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(s.into())))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn add_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `474 + s * (36 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 38_401_000 picoseconds.
		Weight::from_parts(43_707_969, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 1_715
			.saturating_add(Weight::from_parts(54_583, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn rename_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `590 + s * (3 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 18_194_000 picoseconds.
		Weight::from_parts(20_768_301, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 635
			.saturating_add(Weight::from_parts(23_190, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[1, 100]`.
	fn remove_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `637 + s * (35 ±0)`
		//  Estimated: `11037`
		// Minimum execution time: 40_616_000 picoseconds.
		Weight::from_parts(44_077_435, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			// Standard Error: 1_047
			.saturating_add(Weight::from_parts(56_654, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::SuperOf` (r:1 w:1)
	/// Proof: `Identity::SuperOf` (`max_values`: None, `max_size`: Some(114), added: 2589, mode: `MaxEncodedLen`)
	/// Storage: `Identity::SubsOf` (r:1 w:1)
	/// Proof: `Identity::SubsOf` (`max_values`: None, `max_size`: Some(3258), added: 5733, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:0)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	/// The range of component `s` is `[0, 99]`.
	fn quit_sub(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `770 + s * (37 ±0)`
		//  Estimated: `6723`
		// Minimum execution time: 30_647_000 picoseconds.
		Weight::from_parts(33_495_929, 0)
			.saturating_add(Weight::from_parts(0, 6723))
			// Standard Error: 864
			.saturating_add(Weight::from_parts(63_359, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:0 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn add_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 9_598_000 picoseconds.
		Weight::from_parts(10_038_000, 0)
			.saturating_add(Weight::from_parts(0, 0))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	fn remove_username_authority() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `3517`
		// Minimum execution time: 12_603_000 picoseconds.
		Weight::from_parts(13_135_000, 0)
			.saturating_add(Weight::from_parts(0, 3517))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::UsernameAuthorities` (r:1 w:1)
	/// Proof: `Identity::UsernameAuthorities` (`max_values`: None, `max_size`: Some(52), added: 2527, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::PendingUsernames` (r:1 w:0)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn set_username_for() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `79`
		//  Estimated: `11037`
		// Minimum execution time: 82_033_000 picoseconds.
		Weight::from_parts(83_586_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(4))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	/// Storage: `Identity::AccountOfUsername` (r:0 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	fn accept_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `11037`
		// Minimum execution time: 29_906_000 picoseconds.
		Weight::from_parts(30_357_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `Identity::PendingUsernames` (r:1 w:1)
	/// Proof: `Identity::PendingUsernames` (`max_values`: None, `max_size`: Some(85), added: 2560, mode: `MaxEncodedLen`)
	fn remove_expired_approval() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `114`
		//  Estimated: `3550`
		// Minimum execution time: 21_831_000 picoseconds.
		Weight::from_parts(31_329_000, 0)
			.saturating_add(Weight::from_parts(0, 3550))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:0)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:1)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn set_primary_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `11037`
		// Minimum execution time: 23_885_000 picoseconds.
		Weight::from_parts(24_787_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	/// Storage: `Identity::AccountOfUsername` (r:1 w:1)
	/// Proof: `Identity::AccountOfUsername` (`max_values`: None, `max_size`: Some(81), added: 2556, mode: `MaxEncodedLen`)
	/// Storage: `Identity::IdentityOf` (r:1 w:0)
	/// Proof: `Identity::IdentityOf` (`max_values`: None, `max_size`: Some(7572), added: 10047, mode: `MaxEncodedLen`)
	fn remove_dangling_username() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `97`
		//  Estimated: `11037`
		// Minimum execution time: 16_560_000 picoseconds.
		Weight::from_parts(16_931_000, 0)
			.saturating_add(Weight::from_parts(0, 11037))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
