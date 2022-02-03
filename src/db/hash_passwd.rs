use argon2rs::argon2i_simple;

pub fn hash(password: &str) -> String {
    argon2i_simple(&password, "delicious salt")
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

#[test]
    fn it_hashes_a_password() {
        let password = "339513b410525a60adf45d149347bd191e17c2412ff2d87ddfe4a4ddddc12231";
        let hashed = hash("1337");
        assert_eq!(password, hashed);
}