use crate::etrade::builder::Builder;
use crate::{AccountList, BalanceResponse};
use log::debug;
use oagain::BasicConsumer;

mod builder;

#[derive(Debug)]
pub struct ETrade {
    consumer: BasicConsumer,
}

impl ETrade {
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn get_account_list(&mut self) -> crate::Result<AccountList> {
        let response_str = self
            .consumer
            .get(&"https://api.etrade.com/v1/accounts/list".parse()?)?;

        debug!("GET_ACCOUNT_LIST: {}", response_str);
        let account_list: AccountList = quick_xml::de::from_str(&response_str)?;
        Ok(dbg!(account_list))
    }

    pub fn get_balances(
        &mut self,
        account_id_key: impl AsRef<str>,
    ) -> crate::Result<BalanceResponse> {
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
