use crate::database::entities::prelude::*;
use crate::database::entities::{Article, Language, Content};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub id: RecordId<Self>,
    pub article: RecordField<Article>,
    pub language: RecordField<Language>,
    pub published: bool,
    pub title: String,
    pub content: Content,
    pub read_time: (u32, u32)
}

impl Record for ArticleContent {
    const TABLE_NAME: &'static str = "article_content";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl ArticleContent {
    pub fn new(
        article: RecordId<Article>,
        language: RecordId<Language>,
        published: bool,
        title: String,
        content: String
    ) -> Self {
        let content = Content::new(content);
        let read_time = content.read_time();

        Self {
            id: RecordId::<Self>::default(),
            article: RecordField::Id(article),
            language: RecordField::Id(language),
            published,
            title,
            content,
            read_time,
        }
    }
}
