pub mod actions;
pub mod cli;
pub mod coinmarketcap;
pub mod deployed_contracts;
pub mod models;

pub mod log {
    /// Info message.
    pub fn info<T: AsRef<str>>(message: T) {
        prettycli::info(message.as_ref());
    }

    /// Error message.
    pub fn _error<T: AsRef<str>>(message: T) {
        prettycli::error(message.as_ref());
    }
}
