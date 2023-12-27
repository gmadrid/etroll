use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AccountList {
    pub accounts: Accounts,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Accounts {
    pub account: Vec<Account>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub account_id_key: String,
    pub account_name: String,
    pub account_desc: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BalanceResponse {
    pub account_id: String,
    pub account_description: String,

    #[serde(rename = "Computed")]
    pub computed: ComputedBalance,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ComputedBalance {
    cash_balance: f64,
    margin_balance: f64,
    account_balance: f64,
}
