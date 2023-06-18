use sqlite::Connection;

struct DB {
    db_url: String,
    conn: Option<Connection>,
}

impl DB {
    fn new(db_url: String) -> Self {
        DB {
            db_url,
            conn: None,
        }
    }

    pub fn establish_connection(mut self) {
        self.conn = Some(sqlite::open(self.db_url).unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::db::{DB};
    use dotenvy::dotenv;

    // Define your unit tests
    #[test]
    fn test_connection() {
        dotenv().ok();
        use std::env;

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db = DB::new(database_url);
        db.establish_connection();
    }
}
