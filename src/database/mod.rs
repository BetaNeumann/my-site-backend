use std::sync::LazyLock;

mod record;
mod connection;
pub mod entities;

pub use connection::Connection;
pub use record::{Record, RecordId, RecordField, TableName};

pub static DB: LazyLock<Connection> = LazyLock::new(Connection::init);
