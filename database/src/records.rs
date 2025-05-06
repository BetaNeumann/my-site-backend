use time::UtcDateTime;
use crate::record;


record! {
    "language",
    Language {
        description: String,
        acronym: String
    }
}

record! {
    "visitor",
    Visitor {
        name: String,
        created_at: UtcDateTime
    }
}

impl Visitor {
    pub fn new() -> Self {
        let haiku = haikunator::Haikunator::default();
        return Self{name: haiku.haikunate(), created_at: UtcDateTime::now()};
    }
}

record! {
    "article",
    Article {
        slug: String,
        created_at: UtcDateTime
    }
}

record! {
    "article_content",
    ArticleContent {
        article: ArticleWithId,
        language: LanguageWithId,
        title: String,
        content: String,
        published: bool,
        reading_time: i16
    }
}

record! {
    "article_comment",
    ArticleComment {
        article: ArticleWithId,
        visitor: VisitorWithId,
        content: String,
        created_at: UtcDateTime,
        updated_at: UtcDateTime
    }
}

record! {
    "article_view",
    ArticleView {
        article: ArticleWithId,
        visitor: VisitorWithId,
        created_at: UtcDateTime
    }
}
