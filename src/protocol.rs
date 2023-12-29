use serde::Deserialize;
use std::fmt::{Display, Formatter};

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

impl Display for Account {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.account_name, self.account_desc)
    }
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
    pub cash_balance: f64,
    pub margin_balance: f64,
    pub account_balance: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PortfolioResponse {
    pub account_portfolio: Vec<AccountPortfolio>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountPortfolio {
    pub account_id: String,
    pub next: Option<String>,
    pub total_pages: i32,

    #[serde(rename = "Position")]
    pub position: Vec<Position>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    #[serde(rename = "Product")]
    pub product: Product,
    pub symbol_description: String,
    pub position_type: String,
    pub date_acquired: i64,
    pub price_paid: f32,
    pub quantity: i32,
    pub market_value: f32,
    pub total_cost: f32,
    pub total_gain: f32,
    pub total_gain_pct: f32,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Product {
    pub expiry_day: i32,
    pub expiry_month: i32,
    pub expiry_year: i32,
    pub security_type: String,
    pub strike_price: f32,
    pub symbol: String,
}
