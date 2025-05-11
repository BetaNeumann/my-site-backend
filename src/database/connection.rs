use std::ops::Deref;
use surrealdb::{Surreal, engine::remote::ws::Client, method::Content};
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
}

impl Deref for Connection {
    type Target = Surreal<Client>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
