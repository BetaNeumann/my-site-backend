use time::OffsetDateTime;
use serde::{Serialize, Deserialize};

use crate::database::{Record, RecordId, RecordField};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub id: RecordId<Self>,
    pub description: String,
    pub acronym: String
}

impl Record for Language {
    const TABLE_NAME: &'static str = "language";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl Language {
    pub fn new(description: String, acronym: String) -> Self {
        Self {
            id: RecordId::<Self>::new(),
            description,
            acronym
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Visitor {
    pub id: RecordId<Self>,
    pub name: String,
    pub created_at: OffsetDateTime
}

impl Record for Visitor {
    const TABLE_NAME: &'static str = "visitor";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl Visitor {
    pub fn new(name: String) -> Self {
        Self {
            id: RecordId::<Self>::new(),
            name,
            created_at: OffsetDateTime::now_utc()
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Article {
    pub id: RecordId<Self>,
    pub slug: String,
    pub created_at: OffsetDateTime
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
            id: RecordId::<Self>::new(),
            slug,
            created_at: OffsetDateTime::now_utc()
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleContent {
    pub id: RecordId<Self>,
    pub article: RecordField<Article>,
    pub language: RecordField<Language>,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub reading_time: i16
}

impl Record for ArticleContent {
    const TABLE_NAME: &'static str = "article_content";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl ArticleContent {
    pub fn new(
        article: &RecordField<Article>,
        language: &RecordField<Language>,
        title: String,
        content: String,
        published: bool
    ) -> Self {
        Self {
            id: RecordId::<Self>::new(),
            article: RecordField::Id(article.id().clone()),
            language: RecordField::Id(language.id().clone()),
            title,
            content,
            published,
            reading_time: 0
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleComment {
    pub id: RecordId<Self>,
    #[serde(rename = "in")]
    pub article: RecordField<Article>,
    #[serde(rename = "out")]
    pub visitor: RecordField<Visitor>,
    pub content: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime
}

impl Record for ArticleComment {
    const TABLE_NAME: &'static str = "article_comment";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl ArticleComment {
    pub fn new(
        article: &RecordField<Article>,
        visitor: &RecordField<Visitor>,
        content: String
    ) -> Self {
        Self {
            id: RecordId::<Self>::new(),
            article: RecordField::Id(article.id().clone()),
            visitor: RecordField::Id(visitor.id().clone()),
            content,
            created_at: OffsetDateTime::now_utc(),
            updated_at: OffsetDateTime::now_utc()
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArticleView {
    pub id: RecordId<Self>,
    #[serde(rename = "in")]
    pub article: RecordField<Article>,
    #[serde(rename = "out")]
    pub visitor: RecordField<Visitor>,
    pub created_at: OffsetDateTime
}

impl Record for ArticleView {
    const TABLE_NAME: &'static str = "article_view";

    fn id(&self) -> &RecordId<Self> {
        &self.id
    }
}

impl ArticleView {
    pub fn new(
        article: RecordField<Article>,
        visitor: RecordField<Visitor>,
    ) -> Self {
        Self {
            id: RecordId::<Self>::new(),
            article: RecordField::Id(article.id().clone()),
            visitor: RecordField::Id(visitor.id().clone()),
            created_at: OffsetDateTime::now_utc(),
        }
    }
}
