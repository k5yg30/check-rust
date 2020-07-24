use crate::beans::SayHello;

#[derive(Debug)]
pub struct Waitter {
    pub name: String,
     age: u32,
}

impl Waitter {
    pub fn create(name: String, age: u32) -> Self {
        Waitter { name, age }
    }

    pub fn show(&self) {
        println!("name:{},age:{}", self.name, self.age);
    }
}

impl SayHello for Waitter {
    fn do_say_hello(&self) -> i32 {
        println!("I am a Waitter");
        12
    }
}
