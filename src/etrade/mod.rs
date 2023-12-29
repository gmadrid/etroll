use crate::etrade::builder::Builder;
use crate::{AccountList, BalanceResponse, PortfolioResponse, Position, Result};
use log::debug;
use oagain::BasicConsumer;
use url::Url;

mod builder;

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
        Ok(account_list)
    }

    pub fn get_balances(
        &mut self,
        account_id_key: impl AsRef<str>,
        realtime: bool,
    ) -> Result<BalanceResponse> {
        let url_string = format!(
            "https://api.etrade.com/v1/accounts/{}/balance?instType=BROKERAGE&realTimeNAV={}",
            account_id_key.as_ref(),
            if realtime { "true" } else { "false " }
        );
        let response_str = self.consumer.get(&url_string.parse()?)?;

        debug!("GET BALANCES: {}", response_str);
        let balance_response = quick_xml::de::from_str(&response_str)?;
        Ok(balance_response)
    }

    pub fn get_positions(&mut self, account_id_key: impl AsRef<str>) -> Result<Vec<Position>> {
        let url_string = format!(
            "https://api.etrade.com/v1/accounts/{}/portfolio",
            account_id_key.as_ref()
        );
        let mut base_url = Url::parse(&url_string)?;
        base_url.query_pairs_mut().append_pair("count", "250");

        let mut response = Vec::default();
        loop {
            let response_str = self.consumer.get(&base_url)?;
            let mut portfolio_response: PortfolioResponse = quick_xml::de::from_str(&response_str)?;

            response.append(&mut portfolio_response.account_portfolio[0].position);

            if let Some(next) = portfolio_response.account_portfolio[0].next.as_ref() {
                base_url = Url::parse(next)?;
            } else {
                break;
            }
        }

        Ok(response)
    }
}
