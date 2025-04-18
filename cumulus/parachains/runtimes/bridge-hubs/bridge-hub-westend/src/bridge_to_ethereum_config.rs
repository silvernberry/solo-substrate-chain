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

#[cfg(not(feature = "runtime-benchmarks"))]
use crate::XcmRouter;
use crate::{
	xcm_config,
	xcm_config::{TreasuryAccount, UniversalLocation},
	Balances, EthereumInboundQueue, EthereumOutboundQueue, EthereumSystem, MessageQueue, Runtime,
	RuntimeEvent, TransactionByteFee,
};
use parachains_common::{AccountId, Balance};
use snowbridge_beacon_primitives::{Fork, ForkVersions};
use snowbridge_core::{gwei, meth, AllowSiblingsOnly, PricingParameters, Rewards};
use snowbridge_router_primitives::{inbound::MessageToXcm, outbound::EthereumBlobExporter};
use sp_core::H160;
use testnet_parachains_constants::westend::{
	currency::*,
	fee::WeightToFee,
	snowbridge::{EthereumLocation, EthereumNetwork, INBOUND_QUEUE_PALLET_INDEX},
};

use crate::xcm_config::RelayNetwork;
#[cfg(feature = "runtime-benchmarks")]
use benchmark_helpers::DoNothingRouter;
use frame_support::{parameter_types, weights::ConstantMultiplier};
use pallet_xcm::EnsureXcm;
use sp_runtime::{
	traits::{ConstU32, ConstU8, Keccak256},
	FixedU128,
};
use xcm::prelude::{GlobalConsensus, InteriorLocation, Location, Parachain};

pub const SLOTS_PER_EPOCH: u32 = snowbridge_pallet_ethereum_client::config::SLOTS_PER_EPOCH as u32;

/// Exports message to the Ethereum Gateway contract.
pub type SnowbridgeExporter = EthereumBlobExporter<
	UniversalLocation,
	EthereumNetwork,
	snowbridge_pallet_outbound_queue::Pallet<Runtime>,
	snowbridge_core::AgentIdOf,
	EthereumSystem,
>;
use hex_literal::hex;

// Ethereum Bridge
parameter_types! {
	pub storage EthereumGatewayAddress: H160 = H160(hex!("EDa338E4dC46038493b885327842fD3E301CaB39"));
}

parameter_types! {
	pub const CreateAssetCall: [u8;2] = [53, 0];
	pub const CreateAssetDeposit: u128 = (UNITS / 10) + EXISTENTIAL_DEPOSIT;
	pub Parameters: PricingParameters<u128> = PricingParameters {
		exchange_rate: FixedU128::from_rational(1, 400),
		fee_per_gas: gwei(20),
		rewards: Rewards { local: 1 * UNITS, remote: meth(1) },
		multiplier: FixedU128::from_rational(1, 1),
	};
	pub AssetHubFromEthereum: Location = Location::new(1,[GlobalConsensus(RelayNetwork::get()),Parachain(westend_runtime_constants::system_parachain::ASSET_HUB_ID)]);
	pub EthereumUniversalLocation: InteriorLocation = [GlobalConsensus(EthereumNetwork::get())].into();
}
impl snowbridge_pallet_inbound_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Verifier = snowbridge_pallet_ethereum_client::Pallet<Runtime>;
	type Token = Balances;
	#[cfg(not(feature = "runtime-benchmarks"))]
	type XcmSender = XcmRouter;
	#[cfg(feature = "runtime-benchmarks")]
	type XcmSender = DoNothingRouter;
	type ChannelLookup = EthereumSystem;
	type GatewayAddress = EthereumGatewayAddress;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = Runtime;
	type MessageConverter = MessageToXcm<
		CreateAssetCall,
		CreateAssetDeposit,
		ConstU8<INBOUND_QUEUE_PALLET_INDEX>,
		AccountId,
		Balance,
		EthereumSystem,
		EthereumUniversalLocation,
		AssetHubFromEthereum,
	>;
	type WeightToFee = WeightToFee;
	type LengthToFee = ConstantMultiplier<Balance, TransactionByteFee>;
	type MaxMessageSize = ConstU32<2048>;
	type WeightInfo = crate::weights::snowbridge_pallet_inbound_queue::WeightInfo<Runtime>;
	type PricingParameters = EthereumSystem;
	type AssetTransactor = <xcm_config::XcmConfig as xcm_executor::Config>::AssetTransactor;
}

impl snowbridge_pallet_outbound_queue::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type Hashing = Keccak256;
	type MessageQueue = MessageQueue;
	type Decimals = ConstU8<12>;
	type MaxMessagePayloadSize = ConstU32<2048>;
	type MaxMessagesPerBlock = ConstU32<32>;
	type GasMeter = snowbridge_core::outbound::ConstantGasMeter;
	type Balance = Balance;
	type WeightToFee = WeightToFee;
	type WeightInfo = crate::weights::snowbridge_pallet_outbound_queue::WeightInfo<Runtime>;
	type PricingParameters = EthereumSystem;
	type Channels = EthereumSystem;
}

#[cfg(any(feature = "std", feature = "fast-runtime", feature = "runtime-benchmarks", test))]
parameter_types! {
	pub const ChainForkVersions: ForkVersions = ForkVersions {
		genesis: Fork {
			version: hex!("00000000"),
			epoch: 0,
		},
		altair: Fork {
			version: hex!("01000000"),
			epoch: 0,
		},
		bellatrix: Fork {
			version: hex!("02000000"),
			epoch: 0,
		},
		capella: Fork {
			version: hex!("03000000"),
			epoch: 0,
		},
		deneb: Fork {
			version: hex!("04000000"),
			epoch: 0,
		},
		electra: Fork {
			version: hex!("05000000"),
			epoch: 80000000000, // setting to a future epoch for local testing to remain on Deneb.
		},
	};
}

#[cfg(not(any(feature = "std", feature = "fast-runtime", feature = "runtime-benchmarks", test)))]
parameter_types! {
	pub const ChainForkVersions: ForkVersions = ForkVersions {
		genesis: Fork {
			version: hex!("90000069"),
			epoch: 0,
		},
		altair: Fork {
			version: hex!("90000070"),
			epoch: 50,
		},
		bellatrix: Fork {
			version: hex!("90000071"),
			epoch: 100,
		},
		capella: Fork {
			version: hex!("90000072"),
			epoch: 56832,
		},
		deneb: Fork {
			version: hex!("90000073"),
			epoch: 132608,
		},
		electra: Fork {
			version: hex!("90000074"),
			epoch: 222464, // https://github.com/ethereum/EIPs/pull/9322/files
		},
	};
}

impl snowbridge_pallet_ethereum_client::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type ForkVersions = ChainForkVersions;
	type FreeHeadersInterval = ConstU32<SLOTS_PER_EPOCH>;
	type WeightInfo = crate::weights::snowbridge_pallet_ethereum_client::WeightInfo<Runtime>;
}

impl snowbridge_pallet_system::Config for Runtime {
	type RuntimeEvent = RuntimeEvent;
	type OutboundQueue = EthereumOutboundQueue;
	type SiblingOrigin = EnsureXcm<AllowSiblingsOnly>;
	type AgentIdOf = snowbridge_core::AgentIdOf;
	type TreasuryAccount = TreasuryAccount;
	type Token = Balances;
	type WeightInfo = crate::weights::snowbridge_pallet_system::WeightInfo<Runtime>;
	#[cfg(feature = "runtime-benchmarks")]
	type Helper = ();
	type DefaultPricingParameters = Parameters;
	type InboundDeliveryCost = EthereumInboundQueue;
	type UniversalLocation = UniversalLocation;
	type EthereumLocation = EthereumLocation;
}

#[cfg(feature = "runtime-benchmarks")]
pub mod benchmark_helpers {
	use crate::{EthereumBeaconClient, Runtime, RuntimeOrigin};
	use codec::Encode;
	use snowbridge_beacon_primitives::BeaconHeader;
	use snowbridge_pallet_inbound_queue::BenchmarkHelper;
	use sp_core::H256;
	use xcm::latest::{Assets, Location, SendError, SendResult, SendXcm, Xcm, XcmHash};

	impl<T: snowbridge_pallet_ethereum_client::Config> BenchmarkHelper<T> for Runtime {
		fn initialize_storage(beacon_header: BeaconHeader, block_roots_root: H256) {
			EthereumBeaconClient::store_finalized_header(beacon_header, block_roots_root).unwrap();
		}
	}

	pub struct DoNothingRouter;
	impl SendXcm for DoNothingRouter {
		type Ticket = Xcm<()>;

		fn validate(
			_dest: &mut Option<Location>,
			xcm: &mut Option<Xcm<()>>,
		) -> SendResult<Self::Ticket> {
			Ok((xcm.clone().unwrap(), Assets::new()))
		}
		fn deliver(xcm: Xcm<()>) -> Result<XcmHash, SendError> {
			let hash = xcm.using_encoded(sp_io::hashing::blake2_256);
			Ok(hash)
		}
	}

	impl snowbridge_pallet_system::BenchmarkHelper<RuntimeOrigin> for () {
		fn make_xcm_origin(location: Location) -> RuntimeOrigin {
			RuntimeOrigin::from(pallet_xcm::Origin::Xcm(location))
		}
	}
}

pub(crate) mod migrations {
	use alloc::vec::Vec;
	use frame_support::pallet_prelude::*;
	use snowbridge_core::TokenId;

	#[frame_support::storage_alias]
	pub type OldNativeToForeignId<T: snowbridge_pallet_system::Config> = StorageMap<
		snowbridge_pallet_system::Pallet<T>,
		Blake2_128Concat,
		xcm::v4::Location,
		TokenId,
		OptionQuery,
	>;

	/// One shot migration for NetworkId::Westend to NetworkId::ByGenesis(WESTEND_GENESIS_HASH)
	pub struct MigrationForXcmV5<T: snowbridge_pallet_system::Config>(core::marker::PhantomData<T>);
	impl<T: snowbridge_pallet_system::Config> frame_support::traits::OnRuntimeUpgrade
		for MigrationForXcmV5<T>
	{
		fn on_runtime_upgrade() -> Weight {
			let mut weight = T::DbWeight::get().reads(1);

			let translate_westend = |pre: xcm::v4::Location| -> Option<xcm::v5::Location> {
				weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 1));
				Some(xcm::v5::Location::try_from(pre).expect("valid location"))
			};
			snowbridge_pallet_system::ForeignToNativeId::<T>::translate_values(translate_westend);

			let old_keys = OldNativeToForeignId::<T>::iter_keys().collect::<Vec<_>>();
			for old_key in old_keys {
				if let Some(old_val) = OldNativeToForeignId::<T>::get(&old_key) {
					snowbridge_pallet_system::NativeToForeignId::<T>::insert(
						&xcm::v5::Location::try_from(old_key.clone()).expect("valid location"),
						old_val,
					);
				}
				OldNativeToForeignId::<T>::remove(old_key);
				weight.saturating_accrue(T::DbWeight::get().reads_writes(1, 2));
			}

			weight
		}
	}
}
