use std::{convert::From, fmt::Debug, marker::PhantomData, ops::Deref};
use serde::{Serialize, Deserialize, de::DeserializeOwned};
use surrealdb::{RecordIdKey, sql::Id};


pub trait Entity: Sized + Debug + Clone + Serialize + DeserializeOwned {}
impl<T: Sized + Debug + Clone + Serialize + DeserializeOwned + Send> Entity for T {}


pub type TableName = &'static str;
pub trait Record: Entity {
    const TABLE_NAME: TableName;

    fn id(&self) -> &RecordIdKey;
}


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecordId<T: Record> {
    inner: surrealdb::RecordId,
    #[serde(skip)]
    _marker: PhantomData<T>
}

impl<R: Record, T> From<T> for RecordId<R>
where surrealdb::RecordIdKey: From<T> {
    fn from(value: T) -> Self {
        Self { inner: surrealdb::RecordId::from((R::TABLE_NAME, value)), _marker: PhantomData }
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
            inner: surrealdb::RecordId::from((T::TABLE_NAME, Id::ulid().to_raw())),
            _marker: PhantomData
        }
    }
}


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged, bound = "T: Record")]
pub enum RecordField<T: Record> {
    Id(RecordId<T>),
    Full(T),
}

impl<T: Record> RecordField<T> {
    fn id(&self) -> &RecordIdKey {
        match self {
            RecordField::Id(id) => id.key(),
            RecordField::Full(full) => full.id()
        }
    }
}

impl<T: Record> Serialize for RecordField<T> {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            RecordField::Id(id) => id.serialize(serializer),
            RecordField::Full(full) => full.serialize(serializer),
        }
    }
}
