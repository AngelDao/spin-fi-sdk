use near_account_id::AccountId;
use near_crypto::{InMemorySigner, KeyType, SecretKey};

pub fn run(id: &str, seed_phrase: &str) -> Result<InMemorySigner, ()> {
    let account_id: AccountId = id.parse().unwrap();
    let secret_key: SecretKey = seed_phrase.parse().unwrap();
    let signer: InMemorySigner = InMemorySigner::from_secret_key(account_id, secret_key);
    Ok(signer)
}
