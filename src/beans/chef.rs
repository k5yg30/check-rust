use crate::beans::SayHello;

pub struct Chef {
    _food_type: u32,
    _level: u32,
}

impl Chef {
    pub fn new(food_type: u32, level: u32) -> Self {
        Chef {
            _food_type: food_type,
            _level: level,
        }
    }

    pub fn get_food_type(&self) -> u32 {
        self._food_type
    }

    pub fn get_level(&self) -> u32 {
        self._level
    }
}

impl SayHello for Chef {
    fn do_say_hello(&self) -> i32 {
        println!("I am a a chef");
        13
    }
}
