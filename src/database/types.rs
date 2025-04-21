use sqlx::{Pool, Sqlite};

pub type Conn = Pool<Sqlite>;
