use fk_std::read;

extern "C" {
    fn addPassword(password: &'static str);
}

extern "C" {
    fn passwordOptionInput() -> i32;
}

extern "C" {
    fn shortPassword() -> &'static str;
}

pub unsafe fn password_options() -> i32 {
    printfk!("What kind of password would you like to use?\nEnter 1 or 2\n\0");
    printfk!("1) Regular password (8+ characters letters and numbers\n2) Short password (4-6 digit number password)\n\0");
    if passwordOptionInput() == 1 {
        return 1;
    } else if passwordOptionInput() == 2 {
        return 2;
    }
    // else
    printfk!("Didn't enter 1 or 2, defaulting to 1\nYou can change this in settings\n\0");
    return 1;
}

/*  Most mobile devices run on arm cpus and have short passwords
    this option should be selected if you are on a mobile device
    since most mobile passwords are short */
pub fn short_password() {
    printfk!("Enter a 4-6 digit password\n\0");
    let user_password = unsafe { shortPassword() };
    unsafe { addPassword(user_password); }
}
