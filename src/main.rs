pub mod user;

use user::{User, Luser, Color, Point, AlwaysEqual, build_user};

fn main() {

    // Creating an instance of the User struct
    let user1 = build_user("someone1@example.com".to_string(), "someusername1".to_string());
    println!("{:#?}", user1);

    // Changing the value in the email field of a User instance
    let mut user2 = build_user("someone2@example.com".to_string(), "someusername2".to_string());
    user2.email = String::from("anotheremail@example.com");
    println!("{:#?}", user2);

    // Creating a new User instance using one of the values from user1
    let user3 = User{
        email: String::from("anotheremail@example.com"),
        username: user1.username,
        active: user1.active,
        sign_in_count: user1.sign_in_count
    };
    println!("{:#?}", user3);

    // Using struct update syntax 
    let user4 = User{
        email: String::from("anotheremail@example.com"),
        ..user2
    };
    println!("{:#?}", user4);

    // Using Tuple Structs without Named Fields
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{:#?} {:#?}",black, origin);

    // Unit-Like Structs Without Any Fields    
    let subject = AlwaysEqual;
    println!("{:#?}", subject);

    // Struct storing reference using lifetime
    let luser = Luser{
        email: "anotheremail@example.com",
        username: "somelusername",
        active: true,
        sign_in_count: 1
    };
    println!("{:#?}", luser);
}