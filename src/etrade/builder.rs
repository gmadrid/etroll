use crate::etrade::ETrade;
use oagain::{BasicConsumer, ETradePreset};
use std::path::PathBuf;

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
    pub fn build(self) -> crate::Result<ETrade> {
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
