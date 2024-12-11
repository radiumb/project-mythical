
//! Autogenerated weights for `pallet_multibatching`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2024-04-25, STEPS: `500`, REPEAT: `50`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `Mikolas-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! WASM-EXECUTION: `Compiled`, CHAIN: `None`, DB CACHE: `1024`

// Executed Command:
// ./target/release/mythos-node
// benchmark
// pallet
// --steps
// 500
// --template
// ./.maintain/template.hbs
// --repeat
// 50
// --extrinsic
// *
// --wasm-execution
// compiled
// --heap-pages
// 4096
// --pallet
// pallet_multibatching
// --output
// ./pallets/multibatching/src/weights.rs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_multibatching`.
pub trait WeightInfo {
	fn batch(c: u32, s: u32, ) -> Weight;
	fn batch_v2(c: u32, s: u32, ) -> Weight;
}

/// Weights for `pallet_multibatching` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Domain` (r:1 w:0)
	/// Proof: `Multibatching::Domain` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `s` is `[1, 10]`.
	fn batch(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3497`
		// Minimum execution time: 271_000_000 picoseconds.
		Weight::from_parts(285_000_000, 3497)
			// Standard Error: 1_615
			.saturating_add(Weight::from_parts(1_687_188, 0).saturating_mul(c.into()))
			// Standard Error: 160_912
			.saturating_add(Weight::from_parts(27_710_194, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}

	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Domain` (r:1 w:0)
	/// Proof: `Multibatching::Domain` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `s` is `[1, 10]`.
	fn batch_v2(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3497`
		// Minimum execution time: 271_000_000 picoseconds.
		Weight::from_parts(285_000_000, 3497)
			// Standard Error: 1_615
			.saturating_add(Weight::from_parts(1_687_188, 0).saturating_mul(c.into()))
			// Standard Error: 160_912
			.saturating_add(Weight::from_parts(27_710_194, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Domain` (r:1 w:0)
	/// Proof: `Multibatching::Domain` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `s` is `[1, 10]`.
	fn batch(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3497`
		// Minimum execution time: 271_000_000 picoseconds.
		Weight::from_parts(285_000_000, 3497)
			// Standard Error: 1_615
			.saturating_add(Weight::from_parts(1_687_188, 0).saturating_mul(c.into()))
			// Standard Error: 160_912
			.saturating_add(Weight::from_parts(27_710_194, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}

	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Domain` (r:1 w:0)
	/// Proof: `Multibatching::Domain` (`max_values`: Some(1), `max_size`: Some(32), added: 527, mode: `MaxEncodedLen`)
	/// Storage: `Multibatching::Applied` (r:1 w:1)
	/// Proof: `Multibatching::Applied` (`max_values`: None, `max_size`: Some(32), added: 2507, mode: `MaxEncodedLen`)
	/// The range of component `c` is `[1, 1000]`.
	/// The range of component `s` is `[1, 10]`.
	fn batch_v2(c: u32, s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `67`
		//  Estimated: `3497`
		// Minimum execution time: 271_000_000 picoseconds.
		Weight::from_parts(285_000_000, 3497)
			// Standard Error: 1_615
			.saturating_add(Weight::from_parts(1_687_188, 0).saturating_mul(c.into()))
			// Standard Error: 160_912
			.saturating_add(Weight::from_parts(27_710_194, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
