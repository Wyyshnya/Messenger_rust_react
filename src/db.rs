pub mod models;
pub mod schema;
pub mod hash_passwd;


use diesel::pg::{Pg, PgConnection};
use diesel::Connection;
use r2d2_diesel::ConnectionManager;
use r2d2::Pool;
use dotenv::dotenv;

embed_migrations!();

pub type db_pool = Pool<ConnectionManager<PgConnection>>;

pub fn run_migrations(conn: &PgConnection) {
    let _ = diesel_migrations::run_pending_migrations(&*conn);
}

pub fn establish_connection() -> db_pool {
    if cfg!(test) {
        let manager = ConnectionManager::<PgConnection>::new("postgres://wyyshnya:RikiTiki228@localhost:5432/firstry");
        let pool = r2d2::Pool::builder().build(manager).expect("Failed to create db pool");

        run_migrations(&pool.get().unwrap());
        pool
    } else {
        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL").expect("URL db doesn't set");
        let manager = ConnectionManager::<PgConnection>::new(&database_url);
        r2d2::Pool::builder().build(manager).expect("Failed to create db pool")
    }
}