use crate::database::entities::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub id: RecordId<Self>,
    pub description: String,
    pub acronym: String
}

impl Record for Language {
    const TABLE_NAME: &'static str = "language";

    fn id(&self) -> &RecordIdKey {
        self.id.key()
    }
}

impl Language {
    pub fn new(description: String, acronym: String) -> Self {
        Self {
            id: RecordId::<Self>::default(),
            description,
            acronym
        }
    }
}
