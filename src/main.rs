use etroll::{ETrade, Result};

fn main() -> Result<()> {
    env_logger::init();

    let mut etrade = ETrade::builder()
        .use_secrets_file("my_secrets.toml")
        .use_save_file("my_save.toml")
        .build()?;

    let account_list = etrade.get_account_list()?;
    // Basically assuming there's only one.
    let account_id_key = &account_list.accounts.account[0].account_id_key;

    let balances = etrade.get_balances(account_id_key)?;
    Ok(())
}
