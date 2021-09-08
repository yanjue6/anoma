//! The parameters used for the chain's genesis

use std::str::FromStr;

use anoma::ledger::parameters::{EpochDuration, Parameters};
use anoma::ledger::pos::{GenesisValidator, PosParams};
use anoma::types::address::Address;
#[cfg(feature = "dev")]
use anoma::types::key::ed25519::Keypair;
use anoma::types::key::ed25519::PublicKey;
use anoma::types::token;

#[derive(Debug)]
pub struct Genesis {
    pub validators: Vec<Validator>,
    pub parameters: Parameters,
    pub pos_params: PosParams,
}

#[derive(Clone, Debug)]
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
}

pub fn genesis() -> Genesis {
    use anoma::types::address;

    // NOTE When the validator's key changes, tendermint must be reset with
    // `anoma reset` command. To generate a new validator, use the
    // `tests::gen_genesis_validator` below.
    let public_key1: PublicKey = FromStr::from_str("2000000011f678e1ca3d048d518aefc7de9e3e7f37114c6dd70cf3c8aaea684551f79691").unwrap();
    let public_key2: PublicKey = FromStr::from_str("20000000f7ef2722b9cb6d119e9e836feb1839ef5f1403da023fea62ea2c18d93b913b4f").unwrap();
    let public_key3: PublicKey = FromStr::from_str("20000000807ae82a2d3a4ea97b1b29cc32a02b65dc09823bbc67a5596ec9b84b5c4932b5").unwrap();
    let public_key4: PublicKey = FromStr::from_str("20000000828ad4d324bec69149010105a4dca567c2bb2ecb649b49575917ff021530d38c").unwrap();

    let validator1 = Validator {
        pos_data: GenesisValidator {
            address: address::validator1(),
            staking_reward_address: Address::decode("a1qq5qqqqqxaz5vven8yu5gdpng9zrys6ygvurwv3sgsmrvd6xgdzrys6yg4pnwd6z89rrqv2xvjcy9t").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: public_key1.clone(),
            staking_reward_key: public_key1.clone(),
        },
        account_key: public_key1,
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator2 = Validator {
        pos_data: GenesisValidator {
            address: address::validator2(),
            staking_reward_address: Address::decode("a1qq5qqqqq8yerz3zpgcu5vsjxggurxw2rgy65vv3j8pz5v3zygycrg3z9xprrxw2yxfq5vdphd6egxa").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: public_key2.clone(),
            staking_reward_key: public_key2.clone(),
        },
        account_key: public_key2,
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator3 = Validator {
        pos_data: GenesisValidator {
            address: address::validator3(),
            staking_reward_address: Address::decode("a1qq5qqqqqx56r2v2rx3qn2s33ggenjdes8qur2wfj8pzy2d2xx4przvz9gvcnwdpcxgc5x3j9hxjm34").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: public_key3.clone(),
            staking_reward_key: public_key3.clone(),
        },
        account_key: public_key3,
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator4 = Validator {
        pos_data: GenesisValidator {
            address: address::validator4(),
            staking_reward_address: Address::decode("a1qq5qqqqqxazrvw2zx5mrs3pcxum5zvpkgs6rvd2zxpzrqd3hggurydjrg565xw29gepyywzykgzmk0").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: public_key4.clone(),
            staking_reward_key: public_key4.clone(),
        },
        account_key: public_key4,
        non_staked_balance: token::Amount::whole(100_000),
    };

    let validators = vec![validator1, validator2, validator3, validator4];
    let parameters = Parameters {
        epoch_duration: EpochDuration {
            min_num_of_blocks: 10,
            min_duration: anoma::types::time::Duration::minutes(1).into(),
        },
    };
    Genesis {
        validators,
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
