pub mod chef;
pub mod waitter;

pub trait SayHello {
    fn do_say_hello(&self) -> i32;
}

#[derive(Debug)]
pub enum FoodType {
    CHINESE,
    WESTERN,
    JAPANESE,
}
