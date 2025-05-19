use std::{ops::Deref, convert::Into};
use surrealdb::{
    Surreal, RecordIdKey,
    engine::remote::ws::Client,
    method::{Content, Select}
};

use crate::database::Record;

pub struct Connection(Surreal<Client>);
impl Connection {
    pub fn init() -> Self {
        Self(Surreal::init())
    }

    pub fn create<R: Record + 'static>(
        &self,
        record: R,
    ) -> Content<'_, Client, Option<R>> {
        self.0.create(R::TABLE_NAME).content(record)
    }

    pub fn get<R: Record + 'static>(
        &self,
        id: impl Into<RecordIdKey>
    ) -> Select<'_, Client, Option<R>> {
        self.0.select((R::TABLE_NAME, id))
    }
}

impl Deref for Connection {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
