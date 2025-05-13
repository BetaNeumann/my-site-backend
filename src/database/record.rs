use std::{convert::TryFrom, fmt::Debug, marker::PhantomData, ops::Deref, str::FromStr};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use surrealdb::sql::Id;


pub trait Entity: Sized + Debug + Clone + Serialize {}
impl<T: Sized + Debug + Clone + Serialize> Entity for T {}


pub trait Record: Entity {
    const TABLE_NAME: &'static str;

    fn id(&self) -> &RecordId<Self>;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecordId<T: Record> {
    inner: surrealdb::RecordId,
    #[serde(skip)]
    _marker: PhantomData<T>
}

impl<T: Record> TryFrom<&str> for RecordId<T> {
    type Error = uuid::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Self {
            inner: surrealdb::RecordId::from((T::TABLE_NAME, Uuid::from_str(value)?)),
            _marker: PhantomData
        })
    }
}

impl<T: Record> Deref for RecordId<T> {
    type Target = surrealdb::RecordId;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Record> Default for RecordId<T> {
    fn default() -> Self {  
        Self {
            inner: surrealdb::RecordId::from_table_key(T::TABLE_NAME, Id::rand().to_raw()),
            _marker: PhantomData
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum RecordField<T: Record> {
    Id(RecordId<T>),
    Full(T),
}

impl<T: Record> RecordField<T> {
    pub fn id(&self) -> &RecordId<T> {
        match self {
            RecordField::Id(id) => id,
            RecordField::Full(full) => full.id()
        }
    }
}

impl<T: Record + Serialize> Serialize for RecordField<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where S: serde::Serializer {
        match self {
            RecordField::Id(id) => id.serialize(serializer),
            RecordField::Full(full) => full.serialize(serializer),
        }
    }
}
