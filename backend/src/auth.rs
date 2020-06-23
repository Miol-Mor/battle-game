use crate::config::CONFIG;
use crate::errors::ApiError;
use argon2rs::argon2i_simple;

pub fn hash(password: &str) -> String {
    argon2i_simple(&password, &CONFIG.auth_salt)
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}


#[cfg(test)]
mod test {
    use crate::auth::hash;

    #[test]
    fn new() {
        let s = hash("lalala");
        println!("{}", s);
    }
}
