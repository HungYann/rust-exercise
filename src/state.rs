use borsh::{BorshDeserialize, BorshSerialize};

/// Define the type of state stored in accounts
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct GreetingInfo {
    /// number of greetings
    pub message: String,
}
