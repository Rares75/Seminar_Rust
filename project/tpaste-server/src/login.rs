use std::fmt::Error;
fn check_username(username: String) -> bool {
    //TODO:here we check if the username exists in DB

    true
}
fn check_password(password: String) -> bool {
    //TODO:check if the password is correct

    true
}
pub fn login(username: String, password: String) {
    //Result<(), String> {
    println! {"enter login function for: {}",username};
    /*  if check_username(username) && check_password(password) {
        Ok(())
    } else {
        Err(String::from(
            "username or password aren't correct, please try again",
        ))
    }*/
}
