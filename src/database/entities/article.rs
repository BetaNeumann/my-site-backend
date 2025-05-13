use crate::database::entities::prelude::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: RecordId<Self>,
    pub slug: String,
    pub created_at: Datetime
}

impl Record for Article {
    const TABLE_NAME: &'static str = "article";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl Article {
    pub fn new(slug: String) -> Self {
        Self {
            id: RecordId::<Self>::default(),
            slug,
            created_at: Datetime::default()
        }
    }
}
