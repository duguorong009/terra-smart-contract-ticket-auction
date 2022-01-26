use cosmwasm_std::StdError;

#[derive(Debug)]
pub enum TAError {
    UnnecessaryFunds,
    NotAuthorized,
    NotFound,
    InvalidAddress,
    BetNotFinished,
    NotInitialized,
    InsufficientFunds,
    NotStaked,
}

impl From<TAError> for StdError {
    fn from(error: TAError) -> Self {
        match error {
            TAError::UnnecessaryFunds => {
              StdError::generic_err("Ticket Auction Error: Please do not send any unnecessary funds for this transaction")
            }, 
            TAError::NotAuthorized => {
              StdError::generic_err("Not authorized")
            },
            TAError::NotFound => {
              StdError::generic_err("Not found")
            },
            TAError::InvalidAddress => {
              StdError::generic_err("Invalid address")
            },
            TAError::BetNotFinished => {
              StdError::generic_err("Bet not finished yet")
            },
            TAError::NotInitialized => {
              StdError::generic_err("Config not initialized")
            },
            TAError::InsufficientFunds => {
              StdError::generic_err("Insufficient funds")
            },
            TAError::NotStaked => {
              StdError::generic_err("Not staked yet")
            }
        }
    }
}
