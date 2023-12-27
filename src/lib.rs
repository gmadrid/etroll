mod error;
mod protocol;

use oagain::{BasicConsumer, ETradePreset};
use std::path::PathBuf;

pub use error::{ETrollError, Result};
use log::debug;
pub use protocol::*;

#[derive(Debug)]
pub struct ETrade {
    consumer: BasicConsumer,
}

impl ETrade {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn get_account_list(&mut self) -> Result<AccountList> {
        let response_str = self
            .consumer
            .get(&"https://api.etrade.com/v1/accounts/list".parse()?)?;

        debug!("GET_ACCOUNT_LIST: {}", response_str);
        let account_list: AccountList = quick_xml::de::from_str(&response_str)?;
        Ok(dbg!(account_list))
    }

    pub fn get_balances(&mut self, account_id_key: impl AsRef<str>) -> Result<BalanceResponse> {
        let url_string = format!(
            "https://api.etrade.com/v1/accounts/{}/balance?instType=BROKERAGE&realTimeNAV=true",
            account_id_key.as_ref()
        );
        let response_str = self.consumer.get(&url_string.parse()?)?;
        debug!("GET BALANCES: {}", response_str);
        let balance_response = quick_xml::de::from_str(&response_str)?;
        Ok(dbg!(balance_response))
    }
}

#[derive(Debug)]
pub struct Builder {
    secrets_file: PathBuf,
    save_file: PathBuf,
}

impl Default for Builder {
    fn default() -> Self {
        Builder {
            secrets_file: PathBuf::from("etrade_secrets.toml"),
            save_file: PathBuf::from("etrade_save.toml"),
        }
    }
}

impl Builder {
    pub fn build(self) -> Result<ETrade> {
        let consumer = BasicConsumer::builder()
            .use_preset(ETradePreset)?
            .use_secrets_file(self.secrets_file)?
            .use_save_file(self.save_file)?
            .build()?;
        Ok(ETrade { consumer })
    }

    pub fn use_secrets_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.secrets_file = path.into();
        self
    }

    pub fn use_save_file(mut self, path: impl Into<PathBuf>) -> Self {
        self.save_file = path.into();
        self
    }
}
