//! The parameters used for the chain's genesis

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

    use crate::wallet;

    // NOTE When the validator's key changes, tendermint must be reset with
    // `anoma reset` command. To generate a new validator, use the
    // `tests::gen_genesis_validator` below.
    // TODO generate these on machines all get the public key
    let consensus_keypair = wallet::defaults::validator_keypair();
    let account_keypair = wallet::defaults::validator_keypair();
    let staking_reward_keypair = Keypair::from_bytes(&[
        61, 198, 87, 204, 44, 94, 234, 228, 217, 72, 245, 27, 40, 2, 151, 174,
        24, 247, 69, 6, 9, 30, 44, 16, 88, 238, 77, 162, 243, 125, 240, 206,
        111, 92, 66, 23, 105, 211, 33, 236, 5, 208, 17, 88, 177, 112, 100, 154,
        1, 132, 143, 67, 162, 121, 136, 247, 20, 67, 4, 27, 226, 63, 47, 57,
    ])
    .unwrap();

    let validator1 = Validator {
        pos_data: GenesisValidator {
            address: address::validator1(),
            staking_reward_address: Address::decode("a1qq5qqqqqxaz5vven8yu5gdpng9zrys6ygvurwv3sgsmrvd6xgdzrys6yg4pnwd6z89rrqv2xvjcy9t").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: consensus_keypair.public.clone(),
            staking_reward_key: staking_reward_keypair.public.clone(),
        },
        account_key: account_keypair.public.clone(),
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator2 = Validator {
        pos_data: GenesisValidator {
            address: address::validator2(),
            staking_reward_address: Address::decode("a1qq5qqqqq8yerz3zpgcu5vsjxggurxw2rgy65vv3j8pz5v3zygycrg3z9xprrxw2yxfq5vdphd6egxa").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: consensus_keypair.public.clone(),
            staking_reward_key: staking_reward_keypair.public.clone(),
        },
        account_key: account_keypair.public.clone(),
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator3 = Validator {
        pos_data: GenesisValidator {
            address: address::validator3(),
            staking_reward_address: Address::decode("a1qq5qqqqqx56r2v2rx3qn2s33ggenjdes8qur2wfj8pzy2d2xx4przvz9gvcnwdpcxgc5x3j9hxjm34").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: consensus_keypair.public.clone(),
            staking_reward_key: staking_reward_keypair.public.clone(),
        },
        account_key: account_keypair.public.clone(),
        non_staked_balance: token::Amount::whole(100_000),
    };
    let validator4 = Validator {
        pos_data: GenesisValidator {
            address: address::validator4(),
            staking_reward_address: Address::decode("a1qq5qqqqqxazrvw2zx5mrs3pcxum5zvpkgs6rvd2zxpzrqd3hggurydjrg565xw29gepyywzykgzmk0").unwrap(),
            tokens: token::Amount::whole(200_000),
            consensus_key: consensus_keypair.public.clone(),
            staking_reward_key: staking_reward_keypair.public.clone(),
        },
        account_key: account_keypair.public.clone(),
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
