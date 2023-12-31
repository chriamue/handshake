use crate::env::CONTRACT;
use anyhow::anyhow;
use js_sys::Promise;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = getAccounts)]
    pub fn js_get_accounts() -> Promise;
    #[wasm_bindgen(js_name = fetchNumAccounts)]
    pub fn js_fetch_num_accounts(contract: String) -> Promise;
    #[wasm_bindgen(js_name = fetchNumHandshakes)]
    pub fn js_fetch_num_handshakes(contract: String) -> Promise;

    #[wasm_bindgen(js_name = doHandshake)]
    pub fn js_handshake(
        contract: String,
        source: String,
        sender_address: String,
        destination_address: String,
    ) -> Promise;
    #[wasm_bindgen(js_name = doAccountLookup)]
    pub fn js_do_account_lookup(accountAddress: String) -> Promise;
}

/// DTO to communicate with JavaScript
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Account {
    /// account name
    pub name: String,
    /// name of the browser extension
    pub source: String,
    /// the signature type, e.g. "sr25519" or "ed25519"
    pub ty: String,
    /// ss58 formatted address as string. Can be converted into AccountId32 via it's FromStr implementation.
    pub address: String,
}

pub async fn get_accounts() -> Result<Vec<Account>, anyhow::Error> {
    let result = JsFuture::from(js_get_accounts())
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let accounts_str = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    let accounts: Vec<Account> = serde_json::from_str(&accounts_str)?;
    Ok(accounts)
}

pub async fn get_azero_id(account: String) -> Result<String, anyhow::Error> {
    let result = JsFuture::from(js_do_account_lookup(account))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let domain_str = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    Ok(domain_str)
}

pub async fn get_num_accounts() -> Result<String, anyhow::Error> {
    let result = JsFuture::from(js_fetch_num_accounts(CONTRACT.to_string()))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let num_accounts = result
        .as_string()
        .ok_or(anyhow!("Expected a stringified JSON"))?;
    Ok(num_accounts)
}

pub async fn get_num_handshakes() -> Result<String, anyhow::Error> {
    let result = JsFuture::from(js_fetch_num_handshakes(CONTRACT.to_string()))
        .await
        .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let num_handshakes = result
        .as_string()
        .ok_or(anyhow!("Expected a stringified JSON"))?;
    Ok(num_handshakes)
}

pub async fn do_handshake(
    source: String,
    sender_address: String,
    destination_address: String,
) -> Result<String, anyhow::Error> {
    let result = JsFuture::from(js_handshake(
        CONTRACT.to_string(),
        source,
        sender_address,
        destination_address,
    ))
    .await
    .map_err(|js_err| anyhow!("{js_err:?}"))?;
    let result = result
        .as_string()
        .ok_or(anyhow!("Error converting JsValue into String"))?;
    Ok(result)
}
