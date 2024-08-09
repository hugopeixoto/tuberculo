use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

pub struct Sqlite3 {
    connection:
        diesel::r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::sqlite::SqliteConnection>>,
}

impl Sqlite3 {
    pub fn new(path: &str) -> Self {
        let manager =
            diesel::r2d2::ConnectionManager::<diesel::sqlite::SqliteConnection>::new(path);

        let pool = diesel::r2d2::Pool::builder()
            .max_size(1)
            .test_on_check_out(true)
            .build(manager)
            .unwrap();

        {
            let mut connection = pool.get().unwrap();
            connection.run_pending_migrations(MIGRATIONS).unwrap();
        }

        Self { connection: pool }
    }
}

impl crate::database::Database for Sqlite3 {
    fn search(&self, _term: &str) -> Vec<crate::database::Video> {
        vec![]
    }

    fn enqueue(&mut self, url: String) {
        use crate::schema::queue::dsl::*;

        let mut connection = self.connection.get().unwrap();

        diesel::insert_into(queue)
            .values((
                job.eq("download"),
                args.eq(url),
                created_at.eq(diesel::dsl::now),
            ))
            .execute(&mut connection)
            .unwrap();
    }

    fn queue_size(&self) -> usize {
        let mut connection = self.connection.get().unwrap();

        let x: i64 = crate::schema::queue::dsl::queue
            .count()
            .first(&mut connection)
            .unwrap();

        x as usize
    }
}
