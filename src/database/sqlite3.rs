use diesel::prelude::*;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::database::Video;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(diesel::Queryable, diesel::Selectable)]
#[diesel(table_name = crate::schema::queue)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
struct QueueItem {
    pub id: i32,
    pub job: String,
    pub args: String,
    // pub created_at: chrono::NaiveDateTime,
}

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
        let mut connection = self.connection.get().unwrap();

        use crate::schema::videos::dsl;

        dsl::videos
            .limit(100)
            .select(Video::as_select())
            .get_results(&mut connection)
            .unwrap()
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

    fn pop_queue(&mut self) -> Result<crate::database::Job, anyhow::Error> {
        let mut connection = self.connection.get().unwrap();

        use crate::schema::queue::dsl::*;

        let queue_item = diesel::update(queue)
            .set(locked_at.eq(diesel::dsl::now))
            .filter(
                id.eq_any(
                    queue
                        .select(id)
                        .filter(locked_at.is_null())
                        .limit(1)
                        .into_boxed(),
                ),
            )
            .returning(QueueItem::as_select())
            .get_result(&mut connection)
            .map_err(|_x| anyhow::format_err!("nothing in the queue"))?;

        match queue_item.job.as_str() {
            "download" => Ok(crate::database::Job::Download(
                queue_item.id,
                queue_item.args.clone(),
            )),
            x => Err(anyhow::format_err!("Unsupported job type: {}", x)),
        }
    }

    fn done(&mut self, id: i32) {
        let mut connection = self.connection.get().unwrap();

        use crate::schema::queue::dsl;

        diesel::delete(dsl::queue)
            .filter(dsl::id.eq(id))
            .execute(&mut connection)
            .unwrap();
    }
    fn fail(&mut self, id: i32) {
        let mut connection = self.connection.get().unwrap();

        use crate::schema::queue::dsl;

        diesel::update(dsl::queue)
            .filter(dsl::id.eq(id))
            .set(dsl::locked_at.eq(None::<chrono::NaiveDateTime>))
            .execute(&mut connection)
            .unwrap();
    }

    fn store_metadata(&mut self, metadata: &crate::database::Video) -> Result<(), anyhow::Error> {
        let mut connection = self.connection.get().unwrap();

        use crate::schema::videos::dsl;

        diesel::insert_or_ignore_into(dsl::videos)
            .values(metadata)
            .execute(&mut connection)?;
        Ok(())
    }
}
