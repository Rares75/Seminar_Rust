use rusqlite::Result;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Write;

use std::hash::{Hash, Hasher};
use std::iter;
use std::time::{SystemTime, UNIX_EPOCH};
pub fn generate_paste_code() -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

    // Save the current time
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);
    let mut seed = hasher.finish();

    // Generate a 10 character code
    iter::repeat_with(|| {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let idx = (seed % CHARSET.len() as u64) as usize;
        CHARSET[idx] as char
    })
    .take(10)
    .collect()
}

pub fn generate_auth_token() -> String {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut hasher = DefaultHasher::new();
    timestamp.hash(&mut hasher);

    std::process::id().hash(&mut hasher);

    let hash1 = hasher.finish();

    // Generate a second hash for more randomness
    let mut hasher2 = DefaultHasher::new();
    hash1.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    let mut token = String::with_capacity(32);
    write!(&mut token, "{:016x}{:016x}", hash1, hash2).unwrap();

    token
}

/*pub fn read_token_from_file(path: &str) -> std::io::Result<String> {
    let content = fs::read_to_string(path)?;
    Ok(content.trim().to_string())
}*/

/*pub fn save_token_to_file(path: &str, token: &str) -> std::io::Result<()> {
    use std::path::Path;

    // Create the parent directory if it does not exist
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, token)?;
    Ok(())
}*/

pub fn validate_username(username: &str) -> Result<(), String> {
    if username.is_empty() {
        return Err("Username cannot be empty".to_string());
    }

    if username.len() < 3 {
        return Err("Username needs at least 3 characters".to_string());
    }

    if username.len() > 30 {
        return Err("Username cannot be longer than 30 characters".to_string());
    }

    // Check if username only contains digits, letters, and underscore
    if !username.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username can only contain letters, digits, and underscore".to_string());
    }

    Ok(())
}

pub fn validate_password(password: &str) -> Result<String, String> {
    if password.is_empty() {
        return Err("Empty password".to_string());
    }
    if password.len() < 6 {
        return Err("Password is too short".to_string());
    }
    if password.len() > 100 {
        return Err("Password is too long".to_string());
    }

    Ok("Account created successfully".to_string())
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, bcrypt::DEFAULT_COST)
}
