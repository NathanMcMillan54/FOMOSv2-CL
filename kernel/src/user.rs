extern crate libc;

struct User {
    username: u8,
    userpassword: u8,
    defaultuser: bool,
    admin: bool,
}

fn create_user(name: &str, password: &str, loginstartup: bool, admin: bool) {

}

pub fn user_questions() {
    // Ask for name, password, login at startup, and admin

    // defaults
    let name = "User";
    let password = "Password";
    let loginstartup = false;
    let admin = true;
    create_user(name, password, loginstartup, admin);
}
