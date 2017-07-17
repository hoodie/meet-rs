use diesel;
use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;

pub mod schema {
    infer_schema!("env:DATABASE_URL");
}

use self::schema::users;
use self::schema::users::dsl::{ users as all_users,
                                id as user_id,
                                username as user_name,
                                activated as user_activated};

#[derive(Debug, Insertable, Queryable)]
#[table_name = "users"]
pub struct User {
    pub id: Option<i32>,
    username: String,
    password: String,
    email: String,
    bio: String,
    activated: bool,
}

impl User {

    pub fn new() -> NewUser {
        NewUser::default()
    }

    pub fn all(conn: &SqliteConnection) -> Vec<User> {
        all_users.order(users::id.desc())
                 .load::<User>(conn)
                 .unwrap()
    }

    pub fn find_by_name(name: &str, conn: &SqliteConnection) -> Vec<User> {
        all_users.filter(user_name.eq(name))
                 .order(users::id.desc())
                 .load::<User>(conn)
                 .unwrap()
    }

    pub fn activated(conn: &SqliteConnection) -> Vec<User> {
        all_users.filter(user_activated.eq(true))
                 .order(users::id.desc())
                 .load::<User>(conn)
                 .unwrap()
    }

    pub fn activate(&self, activated: bool, conn: &SqliteConnection) -> bool {
        diesel::update(all_users.filter(user_id.eq(self.id)))
               .set(user_activated.eq(activated))
               .execute(conn)
               .is_ok()
    }

    pub fn store(&self, conn: &SqliteConnection) -> bool {
        diesel::insert(self).into(users::table)
                            .execute(conn)
                            .is_ok()
    }

    pub fn delete_with_id(id: i32, conn: &SqliteConnection) -> bool {
        diesel::delete(all_users.find(id)).execute(conn).is_ok()
    }
}

#[derive(Clone, Default)]
pub struct NewUser {
    username: Option<String>,
    password: Option<String>,
    email: Option<String>,
    bio: Option<String>,
}

impl NewUser {
    pub fn username(&mut self, username: &str) -> &mut Self {
        self.username = Some(username.into());
        self
    }

    pub fn password(&mut self, password: &str) -> &mut Self {
        self.password = Some(password.into());
        self
    }

    pub fn email(&mut self, email: &str) -> &mut Self {
        self.email = Some(email.into());
        self
    }

    pub fn bio(&mut self, bio: &str) -> &mut Self {
        self.bio = Some(bio.into());
        self
    }

    pub fn done(&mut self) -> User {
        self.to_owned().into()
    }

}

impl Into<User> for NewUser {
    fn into(self) -> User {
        User {
            id: None,
            username: self.username.unwrap(),
            password: self.password.unwrap(),
            email: self.email.unwrap(),
            bio: self.bio.unwrap(),
            activated: false,

        }
    }
}
