use std::fmt::Error;
fn check_username(_username: String) -> bool {
    // TODO: Check if the username exists in the database
    true
}
fn check_password(_password: String) -> bool {
    // TODO: Check if the password is correct
    true
}
pub fn login(username: String, password: String) {
    // Result<(), String> {
    println!(
        "Entering login function\nUsername received: {}, Password received: {}",
        username, password
    );
}
