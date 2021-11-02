//! The parameters used for the chain's genesis

use std::collections::HashMap;
#[cfg(not(feature = "dev"))]
use std::path::Path;

use anoma::ledger::parameters::Parameters;
use anoma::ledger::pos::{GenesisValidator, PosParams};
use anoma::types::address::Address;
#[cfg(not(feature = "dev"))]
use anoma::types::chain::ChainId;
#[cfg(feature = "dev")]
use anoma::types::key::ed25519::Keypair;
use anoma::types::key::ed25519::PublicKey;
use anoma::types::time::DateTimeUtc;
use anoma::types::{storage, token};
use borsh::{BorshDeserialize, BorshSerialize};
use derivative::Derivative;

/// Genesis configuration file format
pub mod genesis_config {
    use std::array::TryFromSliceError;
    use std::collections::HashMap;
    use std::convert::TryInto;
    use std::path::Path;
    use std::str::FromStr;

    use anoma::ledger::parameters::{EpochDuration, Parameters};
    use anoma::ledger::pos::types::BasisPoints;
    use anoma::ledger::pos::{GenesisValidator, PosParams};
    use anoma::types::address::Address;
    use anoma::types::key::ed25519::{ParsePublicKeyError, PublicKey};
    use anoma::types::time::Rfc3339String;
    use anoma::types::{storage, token};
    use hex;
    use serde::{Deserialize, Serialize};

    use super::{
        EstablishedAccount, Genesis, ImplicitAccount, TokenAccount, Validator,
    };

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct HexString(pub String);

    impl HexString {
        pub fn to_bytes(&self) -> Result<Vec<u8>, HexKeyError> {
            let bytes = hex::decode(self.0.to_owned())?;
            Ok(bytes)
        }

        pub fn to_sha256_bytes(&self) -> Result<[u8; 32], HexKeyError> {
            let bytes = hex::decode(self.0.to_owned())?;
            let slice = bytes.as_slice();
            let array: [u8; 32] = slice.try_into()?;
            Ok(array)
        }

        pub fn to_public_key(&self) -> Result<PublicKey, HexKeyError> {
            let key = PublicKey::from_str(&self.0)?;
            Ok(key)
        }
    }

    #[derive(Debug)]
    pub enum HexKeyError {
        InvalidHexString(hex::FromHexError),
        InvalidSha256(TryFromSliceError),
        InvalidPublicKey(ParsePublicKeyError),
    }

    impl From<hex::FromHexError> for HexKeyError {
        fn from(err: hex::FromHexError) -> Self {
            Self::InvalidHexString(err)
        }
    }

    impl From<ParsePublicKeyError> for HexKeyError {
        fn from(err: ParsePublicKeyError) -> Self {
            Self::InvalidPublicKey(err)
        }
    }

    impl From<TryFromSliceError> for HexKeyError {
        fn from(err: TryFromSliceError) -> Self {
            Self::InvalidSha256(err)
        }
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GenesisConfig {
        // Genesis timestamp
        pub genesis_time: Rfc3339String,
        // Initial validator set
        pub validator: HashMap<String, ValidatorConfig>,
        // Token accounts present at genesis
        pub token: Option<HashMap<String, TokenAccountConfig>>,
        // Established accounts present at genesis
        pub established: Option<HashMap<String, EstablishedAccountConfig>>,
        // Implicit accounts present at genesis
        pub implicit: Option<HashMap<String, ImplicitAccountConfig>>,
        // Protocol parameters
        pub parameters: ParametersConfig,
        // PoS parameters
        pub pos_params: PosParamsConfig,
        // Wasm definitions
        pub wasm: HashMap<String, WasmConfig>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ValidatorConfig {
        // Public key for consensus. (default: generate)
        pub consensus_public_key: Option<HexString>,
        // Public key for validator account. (default: generate)
        pub account_public_key: Option<HexString>,
        // Public key for staking reward account. (default: generate)
        pub staking_reward_public_key: Option<HexString>,
        // Validator address (default: generate).
        pub address: Option<String>,
        // Staking reward account address (default: generate).
        pub staking_reward_address: Option<String>,
        // Total number of tokens held at genesis.
        // XXX: u64 doesn't work with toml-rs!
        pub tokens: u64,
        // Unstaked balance at genesis.
        // XXX: u64 doesn't work with toml-rs!
        pub non_staked_balance: u64,
        // Filename of validator VP. (default: default validator VP)
        pub validator_vp: Option<String>,
        // Filename of staking reward account VP. (default: user VP)
        pub staking_reward_vp: Option<String>,
        // IP:port of the validator. (used in generation only)
        pub net_address: Option<String>,
        /// Matchmaker account's alias, if any
        pub matchmaker_account: Option<String>,
        /// Path to a matchmaker WASM program, if any
        pub matchmaker_code: Option<String>,
        /// Path to a transaction WASM code used by the matchmaker, if any
        pub matchmaker_tx: Option<String>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct TokenAccountConfig {
        // Address of token account (default: generate).
        pub address: Option<String>,
        // Filename of token account VP. (default: token VP)
        pub vp: Option<String>,
        // Initial balances held by accounts defined elsewhere.
        // XXX: u64 doesn't work with toml-rs!
        pub balances: Option<HashMap<String, u64>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EstablishedAccountConfig {
        // Address of established account (default: generate).
        pub address: Option<String>,
        // Filename of established account VP. (default: user VP)
        pub vp: Option<String>,
        // Public key of established account. (default: generate)
        pub public_key: Option<HexString>,
        // Initial storage key values.
        pub storage: Option<HashMap<String, HexString>>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ImplicitAccountConfig {
        // Public key of implicit account (default: generate).
        pub public_key: Option<HexString>,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ParametersConfig {
        // Minimum number of blocks per epoch.
        // XXX: u64 doesn't work with toml-rs!
        min_num_of_blocks: u64,
        // Minimum duration of an epoch (in seconds).
        // TODO: this is i64 because datetime wants it
        min_duration: i64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PosParamsConfig {
        // Maximum number of active validators.
        // XXX: u64 doesn't work with toml-rs!
        max_validator_slots: u64,
        // Pipeline length (in epochs).
        // XXX: u64 doesn't work with toml-rs!
        pipeline_len: u64,
        // Unbonding length (in epochs).
        // XXX: u64 doesn't work with toml-rs!
        unbonding_len: u64,
        // Votes per token (in basis points).
        // XXX: u64 doesn't work with toml-rs!
        votes_per_token: u64,
        // Reward for proposing a block.
        // XXX: u64 doesn't work with toml-rs!
        block_proposer_reward: u64,
        // Reward for voting on a block.
        // XXX: u64 doesn't work with toml-rs!
        block_vote_reward: u64,
        // Portion of a validator's stake that should be slashed on a
        // duplicate vote (in basis points).
        // XXX: u64 doesn't work with toml-rs!
        duplicate_vote_slash_rate: u64,
        // Portion of a validator's stake that should be slashed on a
        // light client attack (in basis points).
        // XXX: u64 doesn't work with toml-rs!
        light_client_attack_slash_rate: u64,
    }

    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct WasmConfig {
        filename: String,
        pub sha256: HexString,
    }

    fn load_validator(
        config: &ValidatorConfig,
        wasm: &HashMap<String, WasmConfig>,
    ) -> Validator {
        let validator_vp_name = config.validator_vp.as_ref().unwrap();
        let validator_vp_config = wasm.get(validator_vp_name).unwrap();
        let reward_vp_name = config.staking_reward_vp.as_ref().unwrap();
        let reward_vp_config = wasm.get(reward_vp_name).unwrap();

        Validator {
            pos_data: GenesisValidator {
                address: Address::decode(&config.address.as_ref().unwrap())
                    .unwrap(),
                staking_reward_address: Address::decode(
                    &config.staking_reward_address.as_ref().unwrap(),
                )
                .unwrap(),
                tokens: token::Amount::whole(config.tokens),
                consensus_key: config
                    .consensus_public_key
                    .as_ref()
                    .unwrap()
                    .to_public_key()
                    .unwrap(),
                staking_reward_key: config
                    .staking_reward_public_key
                    .as_ref()
                    .unwrap()
                    .to_public_key()
                    .unwrap(),
            },
            account_key: config
                .account_public_key
                .as_ref()
                .unwrap()
                .to_public_key()
                .unwrap(),
            non_staked_balance: token::Amount::whole(config.non_staked_balance),
            validator_vp_code_path: validator_vp_config.filename.to_owned(),
            validator_vp_sha256: validator_vp_config
                .sha256
                .to_sha256_bytes()
                .unwrap(),
            reward_vp_code_path: reward_vp_config.filename.to_owned(),
            reward_vp_sha256: reward_vp_config
                .sha256
                .to_sha256_bytes()
                .unwrap(),
        }
    }

    fn load_token(
        config: &TokenAccountConfig,
        wasm: &HashMap<String, WasmConfig>,
    ) -> TokenAccount {
        let token_vp_name = config.vp.as_ref().unwrap();
        let token_vp_config = wasm.get(token_vp_name).unwrap();

        TokenAccount {
            address: Address::decode(&config.address.as_ref().unwrap())
                .unwrap(),
            vp_code_path: token_vp_config.filename.to_owned(),
            vp_sha256: token_vp_config.sha256.to_sha256_bytes().unwrap(),
            balances: config
                .balances
                .as_ref()
                .unwrap_or(&HashMap::default())
                .iter()
                .map(|(address, amount)| {
                    (
                        Address::decode(&address).unwrap(),
                        token::Amount::whole(*amount),
                    )
                })
                .collect(),
        }
    }

    fn load_established(
        config: &EstablishedAccountConfig,
        wasm: &HashMap<String, WasmConfig>,
    ) -> EstablishedAccount {
        let account_vp_name = config.vp.as_ref().unwrap();
        let account_vp_config = wasm.get(account_vp_name).unwrap();

        EstablishedAccount {
            address: Address::decode(&config.address.as_ref().unwrap())
                .unwrap(),
            vp_code_path: account_vp_config.filename.to_owned(),
            vp_sha256: account_vp_config.sha256.to_sha256_bytes().unwrap(),
            public_key: config
                .public_key
                .as_ref()
                .map(|hex| hex.to_public_key().unwrap()),
            storage: config
                .storage
                .as_ref()
                .unwrap_or(&HashMap::default())
                .iter()
                .map(|(address, hex)| {
                    (
                        storage::Key::parse(&address).unwrap(),
                        hex.to_bytes().unwrap(),
                    )
                })
                .collect(),
        }
    }

    fn load_implicit(config: &ImplicitAccountConfig) -> ImplicitAccount {
        ImplicitAccount {
            public_key: config
                .public_key
                .as_ref()
                .unwrap()
                .to_public_key()
                .unwrap(),
        }
    }

    pub fn load_genesis_config(config: GenesisConfig) -> Genesis {
        let wasms = config.wasm;
        let validators = config
            .validator
            .iter()
            .map(|(_name, cfg)| load_validator(cfg, &wasms))
            .collect();
        let token_accounts = config
            .token
            .unwrap_or_default()
            .iter()
            .map(|(_name, cfg)| load_token(cfg, &wasms))
            .collect();
        let established_accounts = config
            .established
            .unwrap_or_default()
            .iter()
            .map(|(_name, cfg)| load_established(cfg, &wasms))
            .collect();
        let implicit_accounts = config
            .implicit
            .unwrap_or_default()
            .iter()
            .map(|(_name, cfg)| load_implicit(cfg))
            .collect();

        let parameters = Parameters {
            epoch_duration: EpochDuration {
                min_num_of_blocks: config.parameters.min_num_of_blocks,
                min_duration: anoma::types::time::Duration::seconds(
                    config.parameters.min_duration,
                )
                .into(),
            },
        };

        let pos_params = PosParams {
            max_validator_slots: config.pos_params.max_validator_slots,
            pipeline_len: config.pos_params.pipeline_len,
            unbonding_len: config.pos_params.unbonding_len,
            votes_per_token: BasisPoints::new(
                config.pos_params.votes_per_token,
            ),
            block_proposer_reward: config.pos_params.block_proposer_reward,
            block_vote_reward: config.pos_params.block_vote_reward,
            duplicate_vote_slash_rate: BasisPoints::new(
                config.pos_params.duplicate_vote_slash_rate,
            ),
            light_client_attack_slash_rate: BasisPoints::new(
                config.pos_params.light_client_attack_slash_rate,
            ),
        };

        let mut genesis = Genesis {
            genesis_time: config.genesis_time.try_into().unwrap(),
            validators,
            token_accounts,
            established_accounts,
            implicit_accounts,
            parameters,
            pos_params,
        };
        genesis.init();
        genesis
    }

    pub fn open_genesis_config(path: impl AsRef<Path>) -> GenesisConfig {
        let config_file = std::fs::read_to_string(path).unwrap();
        toml::from_str(&config_file).unwrap()
    }

    pub fn write_genesis_config(
        config: &GenesisConfig,
        path: impl AsRef<Path>,
    ) {
        let toml = toml::to_string(&config).unwrap();
        std::fs::write(path, toml).unwrap();
    }

    pub fn read_genesis_config(path: impl AsRef<Path>) -> Genesis {
        load_genesis_config(open_genesis_config(path))
    }
}

#[derive(Debug, BorshSerialize, BorshDeserialize)]
#[borsh_init(init)]
pub struct Genesis {
    pub genesis_time: DateTimeUtc,
    pub validators: Vec<Validator>,
    pub token_accounts: Vec<TokenAccount>,
    pub established_accounts: Vec<EstablishedAccount>,
    pub implicit_accounts: Vec<ImplicitAccount>,
    pub parameters: Parameters,
    pub pos_params: PosParams,
}

impl Genesis {
    /// Sort all fields for deterministic encoding
    pub fn init(&mut self) {
        self.validators.sort();
        self.token_accounts.sort();
        self.established_accounts.sort();
        self.implicit_accounts.sort();
    }
}

#[derive(
    Clone,
    Debug,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
/// Genesis validator definition
pub struct Validator {
    /// Data that is used for PoS system initialization
    pub pos_data: GenesisValidator,
    /// Public key associated with the validator account. The default validator
    /// VP will check authorization of transactions from this account against
    /// this key on a transaction signature.
    /// Note that this is distinct from consensus key used in the PoS system.
    pub account_key: PublicKey,
    /// These tokens are no staked and hence do not contribute to the
    /// validator's voting power
    pub non_staked_balance: token::Amount,
    /// Validity predicate code WASM
    pub validator_vp_code_path: String,
    /// Expected SHA-256 hash of the validator VP
    pub validator_vp_sha256: [u8; 32],
    /// Staking reward account code WASM
    pub reward_vp_code_path: String,
    /// Expected SHA-256 hash of the staking reward VP
    pub reward_vp_sha256: [u8; 32],
}

#[derive(
    Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Derivative,
)]
#[derivative(PartialOrd, Ord)]
pub struct EstablishedAccount {
    /// Address
    pub address: Address,
    /// Validity predicate code WASM
    pub vp_code_path: String,
    /// Expected SHA-256 hash of the validity predicate wasm
    pub vp_sha256: [u8; 32],
    /// A public key to be stored in the account's storage, if any
    pub public_key: Option<PublicKey>,
    /// Account's sub-space storage. The values must be borsh encoded bytes.
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub storage: HashMap<storage::Key, Vec<u8>>,
}

#[derive(
    Clone, Debug, BorshSerialize, BorshDeserialize, PartialEq, Eq, Derivative,
)]
#[derivative(PartialOrd, Ord)]
pub struct TokenAccount {
    /// Address
    pub address: Address,
    /// Validity predicate code WASM
    pub vp_code_path: String,
    /// Expected SHA-256 hash of the validity predicate wasm
    pub vp_sha256: [u8; 32],
    /// Accounts' balances of this token
    #[derivative(PartialOrd = "ignore", Ord = "ignore")]
    pub balances: HashMap<Address, token::Amount>,
}

#[derive(
    Clone,
    Debug,
    BorshSerialize,
    BorshDeserialize,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub struct ImplicitAccount {
    /// A public key from which the implicit account is derived. This will be
    /// stored on chain for the account.
    pub public_key: PublicKey,
}

#[cfg(not(feature = "dev"))]
pub fn genesis(base_dir: impl AsRef<Path>, chain_id: &ChainId) -> Genesis {
    let path = base_dir
        .as_ref()
        .join(format!("{}.toml", chain_id.as_str()));
    genesis_config::read_genesis_config(path)
}
#[cfg(feature = "dev")]
pub fn genesis() -> Genesis {
    use anoma::ledger::parameters::EpochDuration;
    use anoma::types::address;

    use crate::wallet;

    let vp_token_path = "vp_token.wasm";
    let vp_user_path = "vp_user.wasm";

    // NOTE When the validator's key changes, tendermint must be reset with
    // `anoma reset` command. To generate a new validator, use the
    // `tests::gen_genesis_validator` below.
    let consensus_keypair = wallet::defaults::validator_keypair();
    let account_keypair = wallet::defaults::validator_keypair();
    let staking_reward_keypair = Keypair::from_bytes(&[
        61, 198, 87, 204, 44, 94, 234, 228, 217, 72, 245, 27, 40, 2, 151, 174,
        24, 247, 69, 6, 9, 30, 44, 16, 88, 238, 77, 162, 243, 125, 240, 206,
        111, 92, 66, 23, 105, 211, 33, 236, 5, 208, 17, 88, 177, 112, 100, 154,
        1, 132, 143, 67, 162, 121, 136, 247, 20, 67, 4, 27, 226, 63, 47, 57,
    ])
    .unwrap();
    let address = wallet::defaults::validator_address();
    let staking_reward_address = Address::decode("atest1v4ehgw36xcersvee8qerxd35x9prsw2xg5erxv6pxfpygd2x89z5xsf5xvmnysejgv6rwd2rnj2avt").unwrap();
    let validator = Validator {
        pos_data: GenesisValidator {
            address,
            staking_reward_address,
            tokens: token::Amount::whole(200_000),
            consensus_key: consensus_keypair.public,
            staking_reward_key: staking_reward_keypair.public,
        },
        account_key: account_keypair.public,
        non_staked_balance: token::Amount::whole(100_000),
        // TODO replace with https://github.com/anoma/anoma/issues/25)
        validator_vp_code_path: vp_user_path.into(),
        validator_vp_sha256: Default::default(),
        reward_vp_code_path: vp_user_path.into(),
        reward_vp_sha256: Default::default(),
    };
    let parameters = Parameters {
        epoch_duration: EpochDuration {
            min_num_of_blocks: 10,
            min_duration: anoma::types::time::Duration::minutes(1).into(),
        },
    };
    let albert = EstablishedAccount {
        address: wallet::defaults::albert_address(),
        vp_code_path: vp_user_path.into(),
        vp_sha256: Default::default(),
        public_key: Some(wallet::defaults::albert_keypair().public),
        storage: HashMap::default(),
    };
    let bertha = EstablishedAccount {
        address: wallet::defaults::bertha_address(),
        vp_code_path: vp_user_path.into(),
        vp_sha256: Default::default(),
        public_key: Some(wallet::defaults::bertha_keypair().public),
        storage: HashMap::default(),
    };
    let christel = EstablishedAccount {
        address: wallet::defaults::christel_address(),
        vp_code_path: vp_user_path.into(),
        vp_sha256: Default::default(),
        public_key: Some(wallet::defaults::christel_keypair().public),
        storage: HashMap::default(),
    };
    let matchmaker = EstablishedAccount {
        address: wallet::defaults::matchmaker_address(),
        vp_code_path: vp_user_path.into(),
        vp_sha256: Default::default(),
        public_key: Some(wallet::defaults::matchmaker_keypair().public),
        storage: HashMap::default(),
    };
    let implicit_accounts = vec![ImplicitAccount {
        public_key: wallet::defaults::daewon_keypair().public,
    }];
    let default_user_tokens = token::Amount::whole(1_000_000);
    let balances: HashMap<Address, token::Amount> = HashMap::from_iter([
        (wallet::defaults::albert_address(), default_user_tokens),
        (wallet::defaults::bertha_address(), default_user_tokens),
        (wallet::defaults::christel_address(), default_user_tokens),
        (wallet::defaults::daewon_address(), default_user_tokens),
    ]);
    let token_accounts = address::tokens()
        .into_iter()
        .map(|(address, _)| TokenAccount {
            address,
            vp_code_path: vp_token_path.into(),
            vp_sha256: Default::default(),
            balances: balances.clone(),
        })
        .collect();
    Genesis {
        genesis_time: DateTimeUtc::now(),
        validators: vec![validator],
        established_accounts: vec![albert, bertha, christel, matchmaker],
        implicit_accounts,
        token_accounts,
        parameters,
        pos_params: PosParams::default(),
    }
}

#[cfg(test)]
pub mod tests {
    use anoma::types::address::testing::gen_established_address;
    use anoma::types::key::ed25519::Keypair;
    use rand::prelude::ThreadRng;
    use rand::thread_rng;

    /// Run `cargo test gen_genesis_validator -- --nocapture` to generate a
    /// new genesis validator address, staking reward address and keypair.
    #[test]
    fn gen_genesis_validator() {
        let address = gen_established_address();
        let staking_reward_address = gen_established_address();
        let mut rng: ThreadRng = thread_rng();
        let keypair = Keypair::generate(&mut rng);
        let staking_reward_keypair = Keypair::generate(&mut rng);
        println!("address: {}", address);
        println!("staking_reward_address: {}", staking_reward_address);
        println!("keypair: {:?}", keypair.to_bytes());
        println!(
            "staking_reward_keypair: {:?}",
            staking_reward_keypair.to_bytes()
        );
    }
}
