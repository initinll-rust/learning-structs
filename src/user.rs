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


#[derive(Debug)]
pub struct Color(pub i32, pub i32, pub i32);
#[derive(Debug)]
pub struct Point(pub i32, pub i32, pub i32);

#[derive(Debug)]
pub struct AlwaysEqual;

pub fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
