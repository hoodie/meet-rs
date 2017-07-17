extern crate meet;
use meet::establish_connection;
use meet::users::User;

fn main() {
    let connection = establish_connection();

    let all_users = User::all(&connection);
    let activated_users = User::activated(&connection);

    println!("activated users:\n{:#?}", activated_users);
    println!("all_users:\n{:#?}", all_users);

}
