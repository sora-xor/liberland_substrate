// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

// File has been modified by Liberland in 2022. All modifications by Liberland are distributed under the MIT license.

// You should have received a copy of the MIT license along with this program. If not, see https://opensource.org/licenses/MIT

use grandpa_primitives::AuthorityId as GrandpaId;
use kitchensink_runtime::{
	constants::currency::*,
	constants::llm::*,
	impls::{IdentityCallFilter, NftsCallFilter, RegistryCallFilter},
	wasm_binary_unwrap, AssetRegistryOfficeConfig, AssetRegistryOfficePalletId,
	AuthorityDiscoveryConfig, BabeConfig, BalancesConfig, Block, CompanyRegistryConfig,
	CompanyRegistryOfficeConfig, CompanyRegistryOfficePalletId, CouncilConfig, DemocracyConfig,
	ElectionsConfig, GenericNetworkId, GrandpaConfig, IdentityOfficeConfig, IdentityOfficePalletId,
	ImOnlineConfig, LLMConfig, LandRegistryOfficeConfig, LandRegistryOfficePalletId,
	LiberlandInitializerConfig, MaxNominations, MetaverseLandRegistryOfficeConfig,
	MetaverseLandRegistryOfficePalletId, SenateConfig, SessionConfig, SessionKeys, SocietyConfig,
	SoraBridgeProviderConfig, StakerStatus, StakingConfig, SubNetworkId, SudoConfig, SystemConfig,
	TechAcc, TechnicalCommitteeConfig, LLM,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::{ChainSpecExtension, Properties};
use sc_service::ChainType;
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{
	crypto::{Ss58Codec, UncheckedInto},
	sr25519, Pair, Public,
};
use sp_runtime::{
	traits::{AccountIdConversion, IdentifyAccount, Verify},
	Perbill,
};

pub use kitchensink_runtime::GenesisConfig;
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn llm_to_lld_10_to_1(grains: Balance) -> Balance {
	let full_merits = grains / GRAINS_IN_LLM;
	let full_dollars = full_merits / 10;
	full_dollars * DOLLARS
}

fn mainnet_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("prefix".into(), 56.into());
	p.insert("network".into(), "Liberland".into());
	p.insert("displayName".into(), "Liberland".into());
	p.insert("tokenSymbol".into(), "LLD".into());
	p.insert("tokenDecimals".into(), 12.into());
	p.insert("standardAccount".into(), "*25519".into());
	p.insert("ss58Format".into(), 56.into());
	p.insert("website".into(), "https://liberland.org".into());
	p
}

/// Mainnet config.
pub fn mainnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Liberland",
		"liberland",
		ChainType::Live,
		mainnet_config_genesis,
		vec![
			// bootnodes - will fill manually after deployment
		],
		None,
		None,
		None,
		Some(mainnet_properties()),
		Default::default(),
	)
}

fn mainnet_config_genesis() -> GenesisConfig {
	/* SPECIAL ACCOUNTS */
	let root_key =
		AccountId::from_ss58check("5D4sqhzUGCSvc6F7Zzx9yh7p5WBbbQfnPxCrirCY2zJ7GDCN").unwrap();
	let pallet_llm_treasury = LLM::get_llm_treasury_account();

	/* VALIDATORS */
	let ll_node_1_stash =
		AccountId::from_ss58check("5DSHUefdTo4d3Vx6byPWgML4fWqXHuAnk7ef9GAouj64aS9x").unwrap();
	let ll_node_2_stash =
		AccountId::from_ss58check("5DtUsuyUY2n2ynewXr87YWzhaKJSamEpvPk7JVqG2aZ6LFVi").unwrap();
	let ll_node_3_stash =
		AccountId::from_ss58check("5CocsjjhgXxP3hsAVYUVnXhgMoqAyc88cadNGEfsN8kKC3F4").unwrap();

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		Balance,
	)> = vec![
		// Liberland Node 1
		(
			ll_node_1_stash.clone(),
			AccountId::from_ss58check("5CiMmBnHTW35RcKyJMYqrRMVDe8Y6Mf6hzpzk2utKrdde7gs").unwrap(),
			array_bytes::hex2array_unchecked(
				"5d11d5d17c244d168fac762b90e2c39c41bcefcc3bf6288e6013cc674037197d",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"5aed4d6b0ce522eb2518ed419f36422fc6362bcea0d7d332ea5a35e2ff281f02",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"0055834ba0620d0c0ce5423f6a8390c61de301fb062386f75dd0c9868994413c",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"a4f0cb3347af88484c8562ca1c54033570cd0d7120b3b397a9f5293456af0224",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
		// Liberland Node 2
		(
			ll_node_2_stash.clone(),
			AccountId::from_ss58check("5CqKDVeckgYUYMbReMLk4QDH84j5b8BdrKRbdGoWj4TQRoK6").unwrap(),
			array_bytes::hex2array_unchecked(
				"ffdc5d43e1e81e7d52792c97c4c3215ad1a4ea521a1c2f2547bab7a8da5af39b",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"aacaf7c0ec104e521da89ae89d8a8470cf3d5df790eca8ac2cd79adaa0a10437",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"7cd34fbd733ded8d1621c37908c81e8370c54f8f62e26f2dfb98afd4ef527618",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"d470b835d4850eb5af4cd9bb53042528c285f116de20999c144453e305b43066",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
		// Liberland Node 3
		(
			ll_node_3_stash.clone(),
			AccountId::from_ss58check("5CDFgvbqyKrmZE9FQpGfdAHZbCR5oRN9h1b77BZo4TPSosfw").unwrap(),
			array_bytes::hex2array_unchecked(
				"851ef2ecc2a98b8ae3f646850f2ef0e8fc79081e774ba354f628510d7ff1fc04",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"aa1522c673f5d32fddc45e8266d52485756ac8abc4a573f7bfd3825fa015c412",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"72000e800c811cb2550ad1d52fef1d9bb43acc885e4bafc047d4f2dd47ffb678",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"3ea7f46f4cd43407c75753a061521a55aa2a0ae82a30ba06fe30d99576841a0d",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
	];

	/* CITIZENSHIPS AND BALANCES */

	let lld_balances = vec![
		(pallet_llm_treasury, 1_330_000 * DOLLARS),
		(ll_node_1_stash, 100 * DOLLARS),
		(ll_node_2_stash, 100 * DOLLARS),
		(ll_node_3_stash, 100 * DOLLARS),
		(
			AccountId::from_ss58check("5GGgzku3kHSnAjxk7HBNeYzghSLsQQQGGznZA7u3h6wZUseo").unwrap(),
			17_300 * DOLLARS,
		),
		(
			AccountId::from_ss58check("5FEaknBkiCR2C436Nz213MwkymeXVJEKE5T7SmUoUSg5rX7X").unwrap(),
			50_000 * DOLLARS,
		),
		(
			AccountId::from_ss58check("5CDpDTBeDdg2KtpgG9WGS92fN4HxpMrSpwtbS6xXke8qU8Xr").unwrap(),
			2_000 * DOLLARS,
		),
	];

	let initial_citizens = vec![
		// (address, total llm, pooled llm)
		(
			AccountId::from_ss58check("5GGgzku3kHSnAjxk7HBNeYzghSLsQQQGGznZA7u3h6wZUseo").unwrap(),
			63_000 * GRAINS_IN_LLM,
			37_000 * GRAINS_IN_LLM,
		),
		(
			// strip citizenship after launch
			AccountId::from_ss58check("5FEaknBkiCR2C436Nz213MwkymeXVJEKE5T7SmUoUSg5rX7X").unwrap(),
			100_000 * GRAINS_IN_LLM,
			0 * GRAINS_IN_LLM,
		),
	];

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig { balances: lld_balances },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), x.6.clone(), StakerStatus::Validator))
				.collect(),
			citizenship_required: true,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		senate: SenateConfig::default(),
		technical_committee: TechnicalCommitteeConfig::default(),
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(kitchensink_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig::default(),
		authority_discovery: AuthorityDiscoveryConfig::default(),
		grandpa: GrandpaConfig::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig { members: vec![], pot: 0, max_members: 999 },
		assets: pallet_assets::GenesisConfig::default(),
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		llm: LLMConfig {
			unpooling_withdrawlock_duration: 3600 * 24 * 30, // 30 days
			unpooling_electionlock_duration: 3600 * 24 * 30,
			_phantom: Default::default(),
		},
		liberland_initializer: LiberlandInitializerConfig {
			citizenship_registrar: Some(IdentityOfficePalletId::get().into_account_truncating()),
			initial_citizens,
			..Default::default()
		},
		company_registry: Default::default(),
		identity_office: Default::default(),
		company_registry_office: Default::default(),
		land_registry_office: Default::default(),
		metaverse_land_registry_office: Default::default(),
		asset_registry_office: Default::default(),
		substrate_bridge_outbound_channel: Default::default(),
		sora_bridge_provider: SoraBridgeProviderConfig {
			register_tech_accounts: vec![(
				GenericNetworkId::Sub(SubNetworkId::Mainnet),
				TechAcc::get(),
			)],
		},
	}
}

fn bastiat_properties() -> Properties {
	let mut p = Properties::new();
	p.insert("prefix".into(), 56.into());
	p.insert("network".into(), "Liberland_testnet".into());
	p.insert("displayName".into(), "Bastiat".into());
	p.insert("tokenSymbol".into(), "LDN".into());
	p.insert("tokenDecimals".into(), 12.into());
	p.insert("standardAccount".into(), "*25519".into());
	p.insert("ss58Format".into(), 56.into());
	p.insert("website".into(), "https://liberland.org".into());
	p
}

/// Bastiat testnet config.
pub fn bastiat_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Bastiat",
		"bastiat",
		ChainType::Live,
		bastiat_testnet_config_genesis,
		vec![
			"/ip4/164.92.254.132/tcp/30333/p2p/12D3KooWFzt9D5Pbza9ahfVjfXfRHsbsQRXmkf64FgY3LrLCSV8N".parse().unwrap(),
			"/ip4/167.99.90.52/tcp/30333/p2p/12D3KooWNUFT7agTugfRJMbVp22Y5MHpUvnjZU6n6rVH1ovHhGxm".parse().unwrap(),
			"/ip4/207.154.237.18/tcp/30333/p2p/12D3KooWBVTvE4yx6WnGCAkBEMxLMDiHbgyh3WKFcBeyhrvJMT3k".parse().unwrap(),
		],
		None,
		None,
		None,
		Some(bastiat_properties()),
		Default::default(),
	)
}

fn bastiat_testnet_config_genesis() -> GenesisConfig {
	let ll_node_1_stash =
		AccountId::from_ss58check("5DLfiq37tePrZoaVNJDPFyDkRa2Lbr7faPMre2omsoNytoq4").unwrap();
	let ll_node_2_stash =
		AccountId::from_ss58check("5CiYBzVkYAJKZ9oa38hnpFuLSiiLxeYyfTb6dzReS1YNaoMy").unwrap();
	let ll_node_3_stash =
		AccountId::from_ss58check("5FnpJZSHMrCCTwukFsbEHVHsVYo5vXGSHnYvhahhtew2jDJL").unwrap();

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
		Balance,
	)> = vec![
		// Liberland Node 1
		(
			ll_node_1_stash.clone(),
			AccountId::from_ss58check("5FyJBpWan9YzAyjwEzKcns4SJYrcJcAb3PKRB7rb8cymgryX").unwrap(),
			array_bytes::hex2array_unchecked(
				"04fd9f3ff2040a822c4ce0275431f9ee5f4c9e5e781742116fa8546c1e0bdb7b",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"d0b72f9ee9b0eeb67450e0a8d71d00e3234ee3b195904e767b60e7a6f589dd55",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"1a6296323683419f4178e64b68476510f2b212261d5c78e28f00ee5e56a42130",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"2062cfe4b566703a1aca8777a40debae985eb0f2dc91dd9a4973c72509f09113",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
		// Liberland Node 2
		(
			ll_node_2_stash.clone(),
			AccountId::from_ss58check("5Df7LyLkNq8BymLP22G7Z696kxao1bMqYLMnGKmPZKqZhrbh").unwrap(),
			array_bytes::hex2array_unchecked(
				"ab73ab7de09146f1af36151e2124076eee1ba327aafac0a995c0bc87d14d1407",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"a8bc8e6b4751db3f3555e2a5e18fdcf71444123460726f1c407645c3ebd0ab4c",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"cccd35bc9fbd981e82ab6217d13fdc361e01be17c7fa6747b479e293aad1d661",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"9af294f1fff0fffa37a0b972edb027a4115ae7567ceeb1da2174bb900a41fc15",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
		// Liberland Node 3
		(
			ll_node_3_stash.clone(),
			AccountId::from_ss58check("5CLUTtAS3w6zLsj7ffZSb7stKKczVUJXHztstmRq1aUSMzHT").unwrap(),
			array_bytes::hex2array_unchecked(
				"054252cb5db71765eac20eed2f40797af88996e88f9168fe136cf78a782af651",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"d08f9c727fe6d7e3fc8132fd021cd673e350a82ef4eda9843bf1d19e038a3d53",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"0ef088bf4da2148f2d119376f31a36f7f60958cecf46f67e8763a57c7f8a3f3b",
			)
			.unchecked_into(),
			array_bytes::hex2array_unchecked(
				"d0f97065a403e04429f9bdefed008b201c171b548f3a166e9aad9f139205076b",
			)
			.unchecked_into(),
			100 * DOLLARS,
		),
	];

	let web3_test1 =
		AccountId::from_ss58check("5GjYePC6HKJGGnEzEZzSvimy6uctuMat4Kr2tjACtKyY9nhT").unwrap();
	let web3_test2 =
		AccountId::from_ss58check("5EqhBxsfDdbddFxcdRPhDBx8V3N2QyQspV5FNfQeT8nFQtj8").unwrap();
	let web3_test3 =
		AccountId::from_ss58check("5CkYuVwK6bRjjaqam76VkPG4xXb1TsmbSQzWrMwaFnQ1nu6z").unwrap();
	let citizen1 =
		AccountId::from_ss58check("5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR").unwrap();
	let registrar_key =
		AccountId::from_ss58check("5FEaknBkiCR2C436Nz213MwkymeXVJEKE5T7SmUoUSg5rX7X").unwrap(); // a.k.a. ministry of interior
	let k = AccountId::from_ss58check("5CDpDTBeDdg2KtpgG9WGS92fN4HxpMrSpwtbS6xXke8qU8Xr").unwrap();
	let d = AccountId::from_ss58check("5DRthHxYaE4tzBMFg4HEkzMTnox7yXceKyirJvGRPmFMorkx").unwrap();
	let m = AccountId::from_ss58check("16cmYqp8953CMh3GoFabGcHZMMGehYyxnNADDUAbFH2Tf5tB").unwrap();
	let n = AccountId::from_ss58check("5GEUDCyZrzPy1A6Kn288pHZFDtVhfYWvYmU1iTUPMg6YSVTE").unwrap();
	let v = AccountId::from_ss58check("5DwWxf1NzMpp4D3jv1KY176DwYRRkKDguprmMw4BjieCX2ZK").unwrap();
	let root_key: AccountId = d.clone();

	let initial_citizens = vec![
		(v, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(n, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(m.clone(), 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(d.clone(), 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(k, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(web3_test1, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(web3_test2, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(web3_test3, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
		(citizen1, 11000 * GRAINS_IN_LLM, 10000 * GRAINS_IN_LLM),
	];

	let mut lld_balances = vec![
		(ll_node_1_stash, 100 * DOLLARS),
		(ll_node_2_stash, 100 * DOLLARS),
		(ll_node_3_stash, 100 * DOLLARS),
	];

	lld_balances.extend(
		initial_citizens
			.iter()
			.map(|(id, merits, _)| (id.clone(), llm_to_lld_10_to_1(*merits))),
	);

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig { balances: lld_balances },
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers: initial_authorities
				.iter()
				.map(|x| (x.0.clone(), x.1.clone(), x.6.clone(), StakerStatus::Validator))
				.collect(),
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig::default(),
		council: CouncilConfig::default(),
		senate: SenateConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: vec![d, m],
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(kitchensink_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig::default(),
		authority_discovery: AuthorityDiscoveryConfig::default(),
		grandpa: GrandpaConfig::default(),
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig { members: vec![], pot: 0, max_members: 999 },
		assets: pallet_assets::GenesisConfig::default(),
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		llm: LLMConfig {
			unpooling_withdrawlock_duration: 3600 * 24 * 30,
			unpooling_electionlock_duration: 3600 * 24 * 30,
			_phantom: Default::default(),
		},
		liberland_initializer: LiberlandInitializerConfig {
			citizenship_registrar: Some(registrar_key),
			initial_citizens,
			..Default::default()
		},
		company_registry: Default::default(),
		identity_office: Default::default(),
		company_registry_office: Default::default(),
		land_registry_office: Default::default(),
		metaverse_land_registry_office: Default::default(),
		asset_registry_office: Default::default(),
		substrate_bridge_outbound_channel: Default::default(),
		sora_bridge_provider: SoraBridgeProviderConfig {
			register_tech_accounts: vec![(
				GenericNetworkId::Sub(SubNetworkId::Mainnet),
				TechAcc::get(),
			)],
		},
	}
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	let initial_authorities: Vec<(AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId)> = vec![
		// Liberland Node 1
		(
			AccountId::from_ss58check("5DLfiq37tePrZoaVNJDPFyDkRa2Lbr7faPMre2omsoNytoq4").unwrap(),
			AccountId::from_ss58check("5FyJBpWan9YzAyjwEzKcns4SJYrcJcAb3PKRB7rb8cymgryX").unwrap(),
			array_bytes::hex2array_unchecked("04fd9f3ff2040a822c4ce0275431f9ee5f4c9e5e781742116fa8546c1e0bdb7b").unchecked_into(),
			array_bytes::hex2array_unchecked("d0b72f9ee9b0eeb67450e0a8d71d00e3234ee3b195904e767b60e7a6f589dd55").unchecked_into(),
			array_bytes::hex2array_unchecked("1a6296323683419f4178e64b68476510f2b212261d5c78e28f00ee5e56a42130").unchecked_into(),
			array_bytes::hex2array_unchecked("2062cfe4b566703a1aca8777a40debae985eb0f2dc91dd9a4973c72509f09113").unchecked_into(),
		),
		// Liberland Node 2
		(
			AccountId::from_ss58check("5CiYBzVkYAJKZ9oa38hnpFuLSiiLxeYyfTb6dzReS1YNaoMy").unwrap(),
			AccountId::from_ss58check("5Df7LyLkNq8BymLP22G7Z696kxao1bMqYLMnGKmPZKqZhrbh").unwrap(),
			array_bytes::hex2array_unchecked("ab73ab7de09146f1af36151e2124076eee1ba327aafac0a995c0bc87d14d1407").unchecked_into(),
			array_bytes::hex2array_unchecked("a8bc8e6b4751db3f3555e2a5e18fdcf71444123460726f1c407645c3ebd0ab4c").unchecked_into(),
			array_bytes::hex2array_unchecked("cccd35bc9fbd981e82ab6217d13fdc361e01be17c7fa6747b479e293aad1d661").unchecked_into(),
			array_bytes::hex2array_unchecked("9af294f1fff0fffa37a0b972edb027a4115ae7567ceeb1da2174bb900a41fc15").unchecked_into(),
		),
		// Liberland Node 3
		(
			AccountId::from_ss58check("5FnpJZSHMrCCTwukFsbEHVHsVYo5vXGSHnYvhahhtew2jDJL").unwrap(),
			AccountId::from_ss58check("5CLUTtAS3w6zLsj7ffZSb7stKKczVUJXHztstmRq1aUSMzHT").unwrap(),
			array_bytes::hex2array_unchecked("054252cb5db71765eac20eed2f40797af88996e88f9168fe136cf78a782af651").unchecked_into(),
			array_bytes::hex2array_unchecked("d08f9c727fe6d7e3fc8132fd021cd673e350a82ef4eda9843bf1d19e038a3d53").unchecked_into(),
			array_bytes::hex2array_unchecked("0ef088bf4da2148f2d119376f31a36f7f60958cecf46f67e8763a57c7f8a3f3b").unchecked_into(),
			array_bytes::hex2array_unchecked("d0f97065a403e04429f9bdefed008b201c171b548f3a166e9aad9f139205076b").unchecked_into(),
		),
	];

	let citizens = vec![
		// F
		AccountId::from_ss58check("5CCi1rPi7cphC6iE9mWkYvbLf57b9N233nFG8hM5zjvYZpLi").unwrap(),
		// V
		AccountId::from_ss58check("5DwWxf1NzMpp4D3jv1KY176DwYRRkKDguprmMw4BjieCX2ZK").unwrap(),
		// N
		AccountId::from_ss58check("5GEUDCyZrzPy1A6Kn288pHZFDtVhfYWvYmU1iTUPMg6YSVTE").unwrap(),
		// Dorian
		AccountId::from_ss58check("5GGgzku3kHSnAjxk7HBNeYzghSLsQQQGGznZA7u3h6wZUseo").unwrap(),
		// M
		AccountId::from_ss58check("5HgUQWZ4HHmivA2kqcXb8TTQVjH11FRphsRj4BBEhBzwUbS8").unwrap(),
		// Citizen 1
		AccountId::from_ss58check("5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR").unwrap(),
		// Web3_Test1
		AccountId::from_ss58check("5GjYePC6HKJGGnEzEZzSvimy6uctuMat4Kr2tjACtKyY9nhT").unwrap(),
		// Web3_Test2
		AccountId::from_ss58check("5EqhBxsfDdbddFxcdRPhDBx8V3N2QyQspV5FNfQeT8nFQtj8").unwrap(),
		// Web3_Test3
		AccountId::from_ss58check("5CkYuVwK6bRjjaqam76VkPG4xXb1TsmbSQzWrMwaFnQ1nu6z").unwrap(),
		// Kacper
		AccountId::from_ss58check("5CDpDTBeDdg2KtpgG9WGS92fN4HxpMrSpwtbS6xXke8qU8Xr").unwrap(),
	];

	let min_citizenship_llm = 5000 * GRAINS_IN_LLM;
	let mut citizens_with_balance: Vec<(AccountId, Balance, Balance)> =
		citizens.iter().map(|id| (id.clone(), 0, 0)).collect();
	citizens_with_balance.extend(vec![
		// Nodes 1-3
		(
			AccountId::from_ss58check("5FyJBpWan9YzAyjwEzKcns4SJYrcJcAb3PKRB7rb8cymgryX").unwrap(),
			min_citizenship_llm,
			min_citizenship_llm,
		),
		(
			AccountId::from_ss58check("5Df7LyLkNq8BymLP22G7Z696kxao1bMqYLMnGKmPZKqZhrbh").unwrap(),
			min_citizenship_llm,
			min_citizenship_llm,
		),
		(
			AccountId::from_ss58check("5CLUTtAS3w6zLsj7ffZSb7stKKczVUJXHztstmRq1aUSMzHT").unwrap(),
			min_citizenship_llm,
			min_citizenship_llm,
		),
	]);

	let registrar_key =
		AccountId::from_ss58check("5G96noBmnpNgpsaVXMsEs7961NU1zUNqQractuCp5R1hKejm").unwrap();
	let root_key: AccountId =
		AccountId::from_ss58check("5GZXCJvjfniCCLmKiyqzXLdwgcSgiQNUtsuFVhrpvfjopShL").unwrap();

	let mut endowed_accounts: Vec<AccountId> = vec![root_key.clone(), registrar_key.clone()];
	endowed_accounts.append(&mut citizens.clone());

	let technical_committee = vec![
		// F
		AccountId::from_ss58check("5CCi1rPi7cphC6iE9mWkYvbLf57b9N233nFG8hM5zjvYZpLi").unwrap(),
		// Dorian
		AccountId::from_ss58check("5GGgzku3kHSnAjxk7HBNeYzghSLsQQQGGznZA7u3h6wZUseo").unwrap(),
		// Kacper
		AccountId::from_ss58check("5CDpDTBeDdg2KtpgG9WGS92fN4HxpMrSpwtbS6xXke8qU8Xr").unwrap(),
	];

	testnet_genesis(
		initial_authorities,
		vec![],
		root_key,
		Some(endowed_accounts),
		Some(vec![]),
		citizens_with_balance,
		Some(technical_committee),
		None,
		vec![],
	)
}

fn properties() -> sc_chain_spec::Properties {
	let mut p = Properties::new();
	p.insert("prefix".into(), 56.into());
	p.insert("network".into(), "liberland".into());
	p.insert("displayName".into(), "Liberland PowellGoHome".into());
	p.insert("tokenSymbol".into(), "LLD".into());
	p.insert("tokenDecimals".into(), 12.into());
	p.insert("standardAccount".into(), "*25519".into());
	p.insert("ss58Format".into(), 56.into());
	p.insert("website".into(), "https://liberland.org".into());
	p
}
/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"PowellGoHome",
		"powell_go_home",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		Some(properties()),
		Default::default(),
	)
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
	TPublic::Pair::from_string(&format!("//{}", seed), None)
		.expect("static values are valid; qed")
		.public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
	AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
	AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
	initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)>,
	initial_nominators: Vec<AccountId>,
	root_key: AccountId,
	endowed_accounts: Option<Vec<AccountId>>,
	council_group: Option<Vec<AccountId>>,
	initial_citizens: Vec<(AccountId, Balance, Balance)>,
	technical_committee: Option<Vec<AccountId>>,
	offices_admin: Option<AccountId>,
	offices_clerks: Vec<AccountId>,
) -> GenesisConfig {
	let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
		vec![
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			get_account_id_from_seed::<sr25519::Public>("Bob"),
			get_account_id_from_seed::<sr25519::Public>("Charlie"),
			get_account_id_from_seed::<sr25519::Public>("Dave"),
			get_account_id_from_seed::<sr25519::Public>("Eve"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie"),
			get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
			get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
			get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
			get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
			get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
			get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
			LandRegistryOfficePalletId::get().into_account_truncating(),
			MetaverseLandRegistryOfficePalletId::get().into_account_truncating(),
			AssetRegistryOfficePalletId::get().into_account_truncating(),
		]
	});

	let council_group: Vec<AccountId> = council_group.unwrap_or(vec![
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		get_account_id_from_seed::<sr25519::Public>("Bob"),
		get_account_id_from_seed::<sr25519::Public>("Charlie"),
	]);

	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
				endowed_accounts.push(x.clone())
			}
		});

	// stakers: all validators and nominators.
	let mut rng = rand::thread_rng();
	let stakers = initial_authorities
		.iter()
		.map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
		.chain(initial_nominators.iter().map(|x| {
			use rand::{seq::SliceRandom, Rng};
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 10_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 1000;
	const INITIAL_STAKE: Balance = 5000 * GRAINS_IN_LLM;

	// Add Prefunded accounts
	let f_ac: Vec<AccountId> = vec![
		array_bytes::hex_n_into_unchecked(
			"061a7f0a43e35d16f330e64c1a4e5000db4ba064fc3630cc4a9e2027899a5a6f",
		), // F
		array_bytes::hex_n_into_unchecked(
			"b86373a2dff0a7b5741fd7e1857de41353fca3b924f14eae5f4c70d69e949150",
		), // N
		array_bytes::hex_n_into_unchecked(
			"ba14fb5a00f052330c9c09e0467bce1d7896edefe92851b893e777aade53f921",
		), // D
		array_bytes::hex_n_into_unchecked(
			"f874b8c112a9bb565e0798d9b5dcfee0fdbd54dd0fcc865c1251a75bd3faee45",
		), // M
		array_bytes::hex_n_into_unchecked(
			"52fd11392742ccf58bcff90c33ca15bdf4bd3416aabcd5d51a654c1f387b6d18",
		), // V
	];

	// rewrite, not to use for loop
	for ac in f_ac.iter() {
		if !endowed_accounts.contains(ac) {
			endowed_accounts.push(ac.clone());
		}
	}

	// endow all citizens.
	initial_citizens.iter().map(|x| &x.0).for_each(|x| {
		if !endowed_accounts.contains(x) {
			endowed_accounts.push(x.clone())
		}
	});

	let technical_committee = technical_committee
		.unwrap_or(endowed_accounts.iter().take((num_endowed_accounts + 1) / 2).cloned().collect());

	let identity_clerks = offices_clerks
		.iter()
		.map(|acc| (acc.clone(), IdentityCallFilter::Judgement))
		.collect();
	let registry_clerks = offices_clerks
		.iter()
		.map(|acc| (acc.clone(), RegistryCallFilter::RegisterOnly))
		.collect();
	let nfts_clerks: Vec<(AccountId, NftsCallFilter)> = offices_clerks
		.iter()
		.map(|acc| (acc.clone(), NftsCallFilter::ManageItems))
		.collect();

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			balances: {
				let mut balalance_initials: Vec<_> = endowed_accounts.iter().cloned().map(|x| (x, ENDOWMENT)).collect();
				balalance_initials.push((TechAcc::get(), ENDOWMENT));
				balalance_initials
			},
		},
		session: SessionConfig {
			keys: initial_authorities
				.iter()
				.map(|x| {
					(
						x.0.clone(),
						x.0.clone(),
						session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
					)
				})
				.collect::<Vec<_>>(),
		},
		staking: StakingConfig {
			validator_count: initial_authorities.len() as u32,
			minimum_validator_count: initial_authorities.len() as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			citizenship_required: false,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: council_group
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, INITIAL_STAKE))
				.collect(),
		},
		council: CouncilConfig::default(),
		senate: SenateConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: technical_committee,
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(kitchensink_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			pot: 0,
			max_members: 999,
		},
		assets: pallet_assets::GenesisConfig {
			// This asset is used by the NIS pallet as counterpart currency.
			assets: vec![(9, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
			..Default::default()
		},
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		llm: Default::default(),
		liberland_initializer: LiberlandInitializerConfig {
			citizenship_registrar: Some(IdentityOfficePalletId::get().into_account_truncating()),
			initial_citizens,
			land_registrar: Some(LandRegistryOfficePalletId::get().into_account_truncating()),
			metaverse_land_registrar: Some(
				MetaverseLandRegistryOfficePalletId::get().into_account_truncating(),
			),
			asset_registrar: Some(AssetRegistryOfficePalletId::get().into_account_truncating()),
		},
		company_registry: CompanyRegistryConfig {
			registries: vec![CompanyRegistryOfficePalletId::get().into_account_truncating()]
				.try_into()
				.unwrap(),
			entities: vec![],
		},
		identity_office: IdentityOfficeConfig {
			admin: offices_admin.clone(),
			clerks: identity_clerks,
		},
		company_registry_office: CompanyRegistryOfficeConfig {
			admin: offices_admin.clone(),
			clerks: registry_clerks,
		},
		land_registry_office: LandRegistryOfficeConfig {
			admin: offices_admin.clone(),
			clerks: nfts_clerks.clone(),
		},
		metaverse_land_registry_office: MetaverseLandRegistryOfficeConfig {
			admin: offices_admin.clone(),
			clerks: nfts_clerks.clone(),
		},
		asset_registry_office: AssetRegistryOfficeConfig {
			admin: offices_admin,
			clerks: nfts_clerks,
		},
		substrate_bridge_outbound_channel: Default::default(),
		sora_bridge_provider: SoraBridgeProviderConfig {
			register_tech_accounts: vec![(
				GenericNetworkId::Sub(SubNetworkId::Mainnet),
				TechAcc::get(),
			)],
		},
	}
}

fn development_config_genesis() -> GenesisConfig {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let bob = get_account_id_from_seed::<sr25519::Public>("Bob");
	let total_llm = 6000 * GRAINS_IN_LLM;
	let locked_llm = 5000 * GRAINS_IN_LLM;
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		alice.clone(),
		None,
		None,
		vec![
			(alice.clone(), total_llm, locked_llm),
			(bob.clone(), total_llm, locked_llm),
			(get_account_id_from_seed::<sr25519::Public>("Charlie"), total_llm, locked_llm),
			(
				AccountId::from_ss58check("5G3uZjEpvNAQ6U2eUjnMb66B8g6d8wyB68x6CfkRPNcno8eR")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Citizen1
			(
				AccountId::from_ss58check("5GGgzku3kHSnAjxk7HBNeYzghSLsQQQGGznZA7u3h6wZUseo")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Dorian
			(
				AccountId::from_ss58check("5GZXCJvjfniCCLmKiyqzXLdwgcSgiQNUtsuFVhrpvfjopShL")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Laissez sudo
			(
				AccountId::from_ss58check("5GjYePC6HKJGGnEzEZzSvimy6uctuMat4Kr2tjACtKyY9nhT")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Web3_Test1
			(
				AccountId::from_ss58check("5EqhBxsfDdbddFxcdRPhDBx8V3N2QyQspV5FNfQeT8nFQtj8")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Web3_Test2
			(
				AccountId::from_ss58check("5CkYuVwK6bRjjaqam76VkPG4xXb1TsmbSQzWrMwaFnQ1nu6z")
					.unwrap(),
				total_llm,
				locked_llm,
			), // Web3_Test3
		],
		None,
		Some(alice.clone()),
		vec![bob.clone()],
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		Some(properties()),
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	let alice = get_account_id_from_seed::<sr25519::Public>("Alice");
	let bob = get_account_id_from_seed::<sr25519::Public>("Bob");
	let total_llm = 6000 * GRAINS_IN_LLM;
	let locked_llm = 5000 * GRAINS_IN_LLM;
	testnet_genesis(
		vec![
			authority_keys_from_seed("Alice"),
			authority_keys_from_seed("Bob"),
			authority_keys_from_seed("Charlie"),
		],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
		None,
		vec![
			(alice.clone(), total_llm, locked_llm),
			(bob.clone(), total_llm, locked_llm),
			(get_account_id_from_seed::<sr25519::Public>("Charlie"), total_llm, locked_llm),
		],
		None,
		Some(alice.clone()),
		vec![bob.clone()],
	)
}

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Local Testnet",
		"local_testnet",
		ChainType::Local,
		local_testnet_genesis,
		vec![],
		None,
		None,
		None,
		Some(properties()),
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
			None,
			vec![],
			None,
			None,
			vec![],
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, transaction_pool, .. } =
				new_full_base(config, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_bastiat_test_net_chain_spec() {
		bastiat_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_mainnet_chain_spec() {
		mainnet_config().build_storage().unwrap();
	}
}
