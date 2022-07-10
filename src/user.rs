#[derive(Debug)]
pub struct User {
    pub active: bool,
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
}

#[derive(Debug)]
pub struct Luser<'a> {
    pub active: bool,
    pub username: &'a str,
    pub email: &'a str,
    pub sign_in_count: u64,
}
