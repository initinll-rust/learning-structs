#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    // Associated functions - not &self - often used for constructors 
    pub fn new(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    // Methods - &self param
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
}

impl Rectangle {
    // Methods - &self param
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}