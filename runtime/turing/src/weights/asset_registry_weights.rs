// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for orml_asset_registry
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-09-13, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! HOSTNAME: `Lauras-MacBook-Pro.local`, CPU: `<UNKNOWN>`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("turing-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/oak-collator
// benchmark
// pallet
// --chain
// turing-dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// orml_asset_registry
// --extrinsic
// *
// --repeat
// 20
// --steps
// 50
// --output
// raw-weights.rs
// --template
// ./.maintain/frame-weight-template.hbs

// Summary:
//:register_asset 25_000_000
//:update_asset 20_000_000

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for orml_asset_registry.
pub trait WeightInfo {
	fn register_asset() -> Weight;
	fn update_asset() -> Weight;
	fn set_asset_location() -> Weight;
}

/// Weights for orml_asset_registry using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> orml_asset_registry::WeightInfo for SubstrateWeight<T> {
	// Storage: AssetRegistry LastAssetId (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:1)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:1)
	fn register_asset() -> Weight {
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: AssetRegistry Metadata (r:1 w:1)
	fn update_asset() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Weight not used in pallet
	fn set_asset_location() -> Weight {
		Weight::zero()
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: AssetRegistry LastAssetId (r:1 w:1)
	// Storage: AssetRegistry Metadata (r:1 w:1)
	// Storage: AssetRegistry LocationToAssetId (r:1 w:1)
	fn register_asset() -> Weight {
		Weight::from_ref_time(25_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: AssetRegistry Metadata (r:1 w:1)
	fn update_asset() -> Weight {
		Weight::from_ref_time(20_000_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Weight not used in pallet
	fn set_asset_location() -> Weight {
		Weight::zero()
	}
}
