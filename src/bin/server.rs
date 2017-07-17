#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate diesel;

extern crate r2d2;
extern crate r2d2_diesel;

extern crate rocket;
use rocket::Rocket;
use rocket::request::{Form, FlashMessage};
use rocket::response::{Flash, Redirect};

extern crate meet;

mod db {
    use std::ops::Deref;

    use r2d2;
    use diesel::sqlite::SqliteConnection;
    use r2d2_diesel::ConnectionManager;

    use rocket::http::Status;
    use rocket::request::{self, FromRequest};
    use rocket::{Request, State, Outcome};

    pub type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

    pub const DATABASE_FILE: &'static str = env!("DATABASE_URL");

    pub fn init_pool() -> Pool {
        let config = r2d2::Config::default();
        let manager = ConnectionManager::<SqliteConnection>::new(DATABASE_FILE);
        r2d2::Pool::new(config, manager).expect("db pool")
    }

    pub struct Conn(pub r2d2::PooledConnection<ConnectionManager<SqliteConnection>>);

    impl Deref for Conn {
        type Target = SqliteConnection;

        #[inline(always)]
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    impl<'a, 'r> FromRequest<'a, 'r> for Conn {
        type Error = ();

        fn from_request(request: &'a Request<'r>) -> request::Outcome<Conn, ()> {
            let pool = request.guard::<State<Pool>>()?;
            match pool.get() {
                Ok(conn) => Outcome::Success(Conn(conn)),
                Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
            }
        }
    }
}

mod users {
    use meet::establish_connection;
    use meet::users::User;
    use super::db;

    #[get("/all")]
    fn all(conn: db::Conn) -> String {
        let activated_users = User::activated(&conn);
        format!("activated users:\n{:#?}", activated_users)
    }
}

fn main() {
    let pool = db::init_pool();
    let conn = if cfg!(test) {
        Some(db::Conn(pool.get().expect("database connection for testing")))
    } else {
        None
    };

    let rocket = rocket::ignite()
        .manage(pool)
        .mount("/users", routes![users::all])
        .launch();
}
