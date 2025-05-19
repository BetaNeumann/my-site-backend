use std::convert::TryFrom;
use serde::{Serialize, Deserialize, de::DeserializeOwned};

use crate::app_err;
use crate::error::AppError;
use crate::database::{Connection, Record};


const MAX_PAGE_LIMIT: u16 = 100;
const DEFAULT_LIMIT: Limit = Limit::new_const(20);


#[derive(Debug, Clone, Deserialize)]
pub struct Pagination {
    key: Option<String>,
    limit: Option<Limit>,
    order: Option<Order>
}

impl Pagination {
    pub async fn query_for<T: Record>(self, db: &Connection) -> Result<PaginationResult<T>, AppError> {
        let limit = self.limit.unwrap_or_default();
        let (direction, op) = self.order.unwrap_or_default().value();

        let content: Vec<T> = match self.key {
            Some(key) => {
                let query = format!(
                    "SELECT * FROM {} WHERE id {} type::thing('{}', $key) ORDER BY id {} LIMIT {}",
                    T::TABLE_NAME, op, T::TABLE_NAME, direction, limit.0
                );

                db.query(query)
                    .bind(("key", key))
                    .await?
                    .take(0)?
            }
            None => {
                let query = format!(
                    "SELECT * FROM {} ORDER BY id {} LIMIT {}",
                    T::TABLE_NAME, direction, limit.0
                );

                db.query(query)
                    .await?
                    .take(0)?
            }
        };

        let prev = content.first().cloned().map(|v| v.id().to_string());
        let next = content.last().cloned().map(|v| v.id().to_string());

        Ok(PaginationResult {
            content,
            prev,
            next,
        })
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(try_from = "u16")]
struct Limit(u16);

impl Limit {
    pub const fn new_const(value: u16) -> Self {
        match value {
            0..=MAX_PAGE_LIMIT => Self(value),
            _ => panic!("Invalid value for const Limit")
        }
    }
}

impl TryFrom<u16> for Limit {
    type Error = AppError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            0..=MAX_PAGE_LIMIT => Ok(Self(value)),
            _ => Err(app_err!(Validation, "Pagination limit exceeds maximum of {MAX_PAGE_LIMIT}"))
        }
    }
}

impl Default for Limit {
    fn default() -> Self { DEFAULT_LIMIT }
}


#[derive(Debug, Clone, Default, Deserialize)]
#[serde(try_from = "String")]
pub enum Order {
    #[default]
    Asc,
    Desc
}

impl Order {
    fn value(&self) -> (&'static str, &'static str) {
        match self {
            Order::Asc => ("ASC", ">"),
            Order::Desc => ("DESC", "<"),
        }
    }
}

impl TryFrom<String> for Order {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.to_uppercase().as_str() {
            "ASC" => Ok(Order::Asc),
            "DESC" => Ok(Order::Desc),
            other => Err(app_err!(Validation, "Invalid input for order: {other}"))
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct PaginationResult<T: Serialize + DeserializeOwned> {
    content: Vec<T>,
    prev: Option<String>,
    next: Option<String>,
}
