//! Allows to take up to 1000 tokens in a single tx without a sig

use anoma_vm_env::vp_prelude::key::ed25519::SignedTxData;
use anoma_vm_env::vp_prelude::*;

enum KeyType<'a> {
    Token(&'a Address),
    Unknown,
}

impl<'a> From<&'a storage::Key> for KeyType<'a> {
    fn from(key: &'a storage::Key) -> KeyType<'a> {
        if let Some(address) = token::is_any_token_balance_key(key) {
            Self::Token(address)
        } else {
            Self::Unknown
        }
    }
}

#[validity_predicate]
fn validate_tx(
    tx_data: Vec<u8>,
    addr: Address,
    keys_changed: HashSet<storage::Key>,
    verifiers: HashSet<Address>,
) -> bool {
    log_string(format!(
        "validate_tx called with user addr: {}, key_changed: {:?}, verifiers: \
         {:?}",
        addr, keys_changed, verifiers
    ));

    // TODO memoize?
    let valid_sig = match SignedTxData::try_from_slice(&tx_data[..]) {
        Ok(tx) => {
            let pk = key::ed25519::get(&addr);
            match pk {
                Some(pk) => verify_tx_signature(&pk, &tx.sig),
                None => false,
            }
        }
        _ => false,
    };

    log_string(format!("signature valid {}, {}", valid_sig, &addr));

    for key in keys_changed.iter() {
        let is_valid = match KeyType::from(key) {
            KeyType::Token(owner) => {
                let key = key.to_string();
                let pre: token::Amount = read_pre(&key).unwrap_or_default();
                let post: token::Amount =
                    read_post(&key).unwrap_or_default();
                let change = post.change() - pre.change();
                if owner == &addr {
                    // debit has to signed, credit doesn't
                    // allow debit for up to 1000 tokens in a single tx
                    let valid = !(change < 0 && !valid_sig) || change >= -1_000_000_000;
                    log_string(format!(
                        "token key: {}, change: {}, transfer_valid_sig: {}, \
                         valid modification: {}",
                        key, change, valid_sig, valid
                    ));
                    valid
                } else {
                    log_string(format!(
                        "Token: key {} is not of owner, transfer_valid_sig \
                         {}, owner: {}, address: {}",
                        key, valid_sig, owner, addr
                    ));
                    valid_sig
                }
            }
            KeyType::Unknown => {
                log_string(format!(
                    "Unknown key modified, valid sig {}",
                    valid_sig
                ));
                valid_sig
            }
        };
        if !is_valid {
            log_string(format!("key {} modification failed vp", key));
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use anoma_tests::vp::*;

    use super::*;

    /// Test that no-op transaction (i.e. no storage modifications) is deemed
    /// valid.
    #[test]
    fn test_no_op_transaction() {
        let mut env = TestVpEnv::default();
        init_vp_env(&mut env);

        let tx_data: Vec<u8> = vec![];
        let addr: Address = env.addr;
        let keys_changed: HashSet<storage::Key> = HashSet::default();
        let verifiers: HashSet<Address> = HashSet::default();

        let valid = validate_tx(tx_data, addr, keys_changed, verifiers);

        assert!(valid);
    }
}
