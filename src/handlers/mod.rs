pub mod auth;
pub mod protected;

pub use auth::{login, register};
pub use protected::get_protected_data;
