use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match classic_error::throw_account_not_found() {
        Err(e) => {
            println!("classic_error Error: {}", e);
            println!(
                "classic_error Caused by {}",
                e.source().ok_or(classic_error::AccountAuthError)?
            );
        }
        _ => {
            println!("classic_error OK")
        }
    }

    match use_from::throw_account_not_found() {
        Err(e) => {
            println!("use_from Error: {}", e);
        }
        _ => {
            println!("use_from OK")
        }
    }

    match use_thiserror::thiserror() {
        Err(e) => println!("thiserror Error: {}", e),
        _ => println!("thiserror OK"),
    }

    match use_anyhow::anyhow() {
        Err(e) => println!("anyhow Error: {}", e),
        _ => println!("anyhow OK"),
    }

    Ok(())
}

pub mod classic_error {

    #[derive(Debug)]
    pub struct AccountNotFoundError {
        err: AccountAuthError,
    }

    impl std::fmt::Display for AccountNotFoundError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "AccountNotFoundError")
        }
    }

    impl std::error::Error for AccountNotFoundError {
        fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
            Some(&self.err)
        }
    }

    pub fn throw_account_not_found() -> Result<(), AccountNotFoundError> {
        Err(AccountNotFoundError {
            err: AccountAuthError,
        })
    }

    #[derive(Debug)]
    pub struct AccountAuthError;

    impl std::fmt::Display for AccountAuthError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "AccountAuthError")
        }
    }

    impl std::error::Error for AccountAuthError {}

    pub fn throw_account_auth() -> Result<(), AccountAuthError> {
        Err(AccountAuthError)
    }
}

pub mod use_from {

    #[derive(Debug)]
    pub struct AccountNotFoundError;

    impl std::fmt::Display for AccountNotFoundError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "AccountNotFoundError")
        }
    }

    impl std::error::Error for AccountNotFoundError {}

    pub fn throw_account_not_found() -> Result<(), AccountNotFoundError> {
        Err(AccountNotFoundError)
    }

    #[derive(Debug)]
    pub struct AccountAuthError;

    impl std::fmt::Display for AccountAuthError {
        fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(fmt, "AccountAuthError")
        }
    }

    impl std::error::Error for AccountAuthError {}

    pub fn throw_account_auth() -> Result<(), AccountAuthError> {
        Err(AccountAuthError)
    }

    // 使用 From 特制显示的转换错误的话，
    // 需要main的返回值写成 `Result<(), use_from::AccountAuthError>` 才行
    impl From<AccountNotFoundError> for AccountAuthError {
        fn from(_: AccountNotFoundError) -> Self {
            AccountAuthError
        }
    }
}

pub mod use_thiserror {
    #[derive(Debug, thiserror::Error)]
    pub enum Error {
        #[error(transparent)]
        VarError(#[from] std::env::VarError),
        #[error(transparent)]
        IoError(#[from] std::io::Error),
    }

    pub fn thiserror() -> Result<(), Error> {
        let _ = std::env::var("WORK_TIME")?;
        // let _ = std::fs::read_to_string("/dev/null")?;
        Ok(())
    }
}

pub mod use_anyhow {
    pub fn anyhow() -> anyhow::Result<()> {
        // let _ = std::env::var("WORK_TIME")?;
        let _ = std::fs::read_to_string("/dev/null")?;
        Ok(())
    }
}
