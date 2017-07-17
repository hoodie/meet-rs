extern crate meet;
use meet::establish_connection;
use meet::users::User;

use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("What user would you like to activate?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)];

    let mut results = User::find_by_name(&username, &connection);

    if let Some(user) = results.get_mut(0) {
        user.activate(true, &connection);
    }
    let all_users = User::all(&connection);
    println!("all_users:\n{:#?}", all_users);

}
