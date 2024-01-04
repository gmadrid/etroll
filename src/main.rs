use inquire::Select;
use std::fmt::Display;
use std::path::PathBuf;

use argh::FromArgs;
use directories::ProjectDirs;
use etroll::ETrade;
use etroll::Result;
use log::info;
use tabled::settings::Style;
use tabled::{Table, Tabled};

#[derive(FromArgs, PartialEq, Debug)]
/// Top-level command
struct TopLevel {
    #[argh(subcommand)]
    nested: SubCommandEnum,
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand)]
enum SubCommandEnum {
    Balances(Balances),
    Portfolio(Portfolio),
}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "balances")]
/// Display the current account balances.
struct Balances {}

#[derive(FromArgs, PartialEq, Debug)]
#[argh(subcommand, name = "portfolio")]
/// Display the current portfolio contents.
struct Portfolio {}

// Returns (account_id, account_id_key) for the chosen account.
fn determine_account(etrade: &mut ETrade) -> Result<(String, String)> {
    let mut account_list = etrade.get_account_list()?;

    if account_list.accounts.account.len() == 1 {
        // unwrap: okay because it has one element.
        let account = account_list.accounts.account.pop().unwrap();
        info!("Using single account: {:?}", account);
        return Ok((account.account_id, account.account_id_key));
    }

    let Ok(account) = Select::new("Choose an account", account_list.accounts.account).prompt()
    else {
        panic!("Failed to choose an account");
    };
    return Ok((account.account_id, account.account_id_key));
}

#[derive(Tabled, Debug)]
struct LabelledRow<'a, T: Display> {
    label: &'a str,
    value: T,
}

impl<'a, T: Display> LabelledRow<'a, T> {
    fn row(label: &'a str, value: T) -> LabelledRow<'a, T> {
        LabelledRow { label, value }
    }
}

fn portfolio_cmd(etrade: &mut ETrade, _portfolio: Portfolio) -> Result<()> {
    let (_, account_id_key) = determine_account(etrade)?;
    let portfolio = etrade.get_positions(account_id_key)?;

    dbg!(&portfolio[0..3]);

    Ok(())
}

fn balances_cmd(etrade: &mut ETrade, _balances: Balances) -> Result<()> {
    let (_, account_id_key) = determine_account(etrade)?;
    let balances = etrade.get_balances(&account_id_key, true)?;

    let output_vec: Vec<LabelledRow<String>> = vec![
        LabelledRow::row("Cash Balance", balances.computed.cash_balance.to_string()),
        LabelledRow::row(
            "Margin Balance",
            balances.computed.margin_balance.to_string(),
        ),
        LabelledRow::row(
            "Account Balance",
            balances.computed.account_balance.to_string(),
        ),
    ];

    println!(
        "{}",
        Table::new(output_vec).with(Style::rounded()).to_string()
    );

    Ok(())
}

fn default_secrets_file() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("my_secrets.toml")
    } else {
        ProjectDirs::from("com", "scrawlsoft", "etroll")
            // unwrap: this should fail if it fails
            .unwrap()
            .config_dir()
            .join("secrets.toml")
    }
}

fn default_config_save_file() -> PathBuf {
    if cfg!(debug_assertions) {
        PathBuf::from("my_save.toml")
    } else {
        ProjectDirs::from("com", "scrawlsoft", "etroll")
            // unwrap: this should fail if it fails
            .unwrap()
            .config_local_dir()
            .join("save.toml")
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let cmd_line: TopLevel = argh::from_env();

    let mut etrade = ETrade::builder()
        .use_secrets_file(default_secrets_file())
        .use_save_file(default_config_save_file())
        .build()?;

    let TopLevel { nested } = cmd_line;

    match nested {
        SubCommandEnum::Balances(balances) => balances_cmd(&mut etrade, balances)?,
        SubCommandEnum::Portfolio(portfolio) => portfolio_cmd(&mut etrade, portfolio)?,
    }

    // let account_list = etrade.get_account_list()?;
    //
    // let Ok(account) = Select::new("Choose an account", account_list.accounts.account).prompt()
    // else {
    //     panic!("Failed to choose an account");
    // };
    //
    // let balances = etrade.get_balances(&account.account_id_key, true);
    Ok(())
}
