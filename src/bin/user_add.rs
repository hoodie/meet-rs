extern crate meet;
use meet::establish_connection;
use meet::users::User;

use std::io::{stdin};

fn main() {
    let connection = establish_connection();

    println!("What would you like your username to be?");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)];
    // Drop the newline character

    println!("What would you like your password to be?");
    let mut password = String::new();
    stdin().read_line(&mut password).unwrap();
    let password = &password[..(password.len() - 1)];
    // Drop the newline character

    let user = User::new().username(&username)
                          .password(&password)
                          .bio("generated")
                          .email(&format!("{}@example.com", username))
                          .done();
    
    User::from(user).store(&connection);


}
