// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_xcm_benchmarks::fungible`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 32.0.0
//! DATE: 2025-03-12, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `dd489642b81c`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! WASM-EXECUTION: Compiled, CHAIN: None, DB CACHE: 1024

// Executed Command:
// target/production/polkadot-parachain
// benchmark
// pallet
// --steps=50
// --repeat=20
// --extrinsic=*
// --wasm-execution=compiled
// --heap-pages=4096
// --json-file=/builds/parity/mirrors/polkadot-sdk/.git/.artifacts/bench.json
// --pallet=pallet_xcm_benchmarks::fungible
// --chain=asset-hub-rococo-dev
// --header=./cumulus/file_header.txt
// --template=./cumulus/templates/xcm-bench-template.hbs
// --output=./cumulus/parachains/runtimes/assets/asset-hub-rococo/src/weights/xcm/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weights for `pallet_xcm_benchmarks::fungible`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo<T> {
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:1 w:1)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:1)
	// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `AssetsFreezer::Freezes` (r:1 w:1)
	// Proof: `AssetsFreezer::Freezes` (`max_values`: None, `max_size`: Some(87), added: 2562, mode: `MaxEncodedLen`)
	pub fn withdraw_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `493`
		//  Estimated: `3675`
		// Minimum execution time: 56_861_000 picoseconds.
		Weight::from_parts(58_296_000, 3675)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(5))
	}
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:2 w:2)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:0)
	// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	pub fn transfer_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `392`
		//  Estimated: `6208`
		// Minimum execution time: 51_788_000 picoseconds.
		Weight::from_parts(53_051_000, 6208)
			.saturating_add(T::DbWeight::get().reads(5))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:2 w:2)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `AssetsFreezer::FrozenBalances` (r:1 w:0)
	// Proof: `AssetsFreezer::FrozenBalances` (`max_values`: None, `max_size`: Some(84), added: 2559, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:3 w:3)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	pub fn transfer_reserve_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `567`
		//  Estimated: `8799`
		// Minimum execution time: 127_591_000 picoseconds.
		Weight::from_parts(131_326_000, 8799)
			.saturating_add(T::DbWeight::get().reads(12))
			.saturating_add(T::DbWeight::get().writes(7))
	}
	pub fn reserve_asset_deposited() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 1_310_000 picoseconds.
		Weight::from_parts(1_454_000, 0)
	}
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	pub fn initiate_reserve_withdraw() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `246`
		//  Estimated: `6196`
		// Minimum execution time: 108_706_000 picoseconds.
		Weight::from_parts(111_596_000, 6196)
			.saturating_add(T::DbWeight::get().reads(7))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	pub fn receive_teleported_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_985_000 picoseconds.
		Weight::from_parts(3_195_000, 0)
	}
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:1 w:1)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	pub fn deposit_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `280`
		//  Estimated: `3675`
		// Minimum execution time: 32_454_000 picoseconds.
		Weight::from_parts(33_452_000, 3675)
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:1 w:1)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:1 w:1)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	pub fn deposit_reserve_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `3819`
		// Minimum execution time: 70_617_000 picoseconds.
		Weight::from_parts(72_500_000, 3819)
			.saturating_add(T::DbWeight::get().reads(8))
			.saturating_add(T::DbWeight::get().writes(4))
	}
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `Assets::Asset` (r:1 w:0)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	pub fn initiate_teleport() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `3819`
		// Minimum execution time: 52_909_000 picoseconds.
		Weight::from_parts(54_376_000, 3819)
			.saturating_add(T::DbWeight::get().reads(6))
			.saturating_add(T::DbWeight::get().writes(1))
	}
	// Storage: `Assets::Asset` (r:1 w:1)
	// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(210), added: 2685, mode: `MaxEncodedLen`)
	// Storage: `Assets::Account` (r:1 w:1)
	// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(134), added: 2609, mode: `MaxEncodedLen`)
	// Storage: `System::Account` (r:2 w:2)
	// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	// Storage: `ParachainSystem::UpwardDeliveryFeeFactor` (r:1 w:0)
	// Proof: `ParachainSystem::UpwardDeliveryFeeFactor` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::VersionDiscoveryQueue` (r:1 w:1)
	// Proof: `PolkadotXcm::VersionDiscoveryQueue` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `PolkadotXcm::SafeXcmVersion` (r:1 w:0)
	// Proof: `PolkadotXcm::SafeXcmVersion` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	pub fn initiate_transfer() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `354`
		//  Estimated: `6196`
		// Minimum execution time: 87_524_000 picoseconds.
		Weight::from_parts(91_889_000, 6196)
			.saturating_add(T::DbWeight::get().reads(9))
			.saturating_add(T::DbWeight::get().writes(5))
	}
}
