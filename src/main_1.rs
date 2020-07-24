use std::any::Any;

use crate::my_lib::a::{a_check, add, mult_value};
use crate::my_lib::{Double, Int, TOTAL_COUNT};
use crate::my_user::swim::{CatSwim, DogSwim, Swim};
use crate::my_user::user::User;
use crate::my_user::worker::Worker;

mod my_lib;
mod my_user;

fn main1() {
    let _dval2: Double = 887.678;
    let bb: Int = 987 + TOTAL_COUNT;
    println!("bb:{},d2:{}", bb, _dval2);
    println!("bb.type:{:?}", bb.type_id());

    a_check();

    let user3 = User::create("lcp333".to_string(), 999);
    user3.print_info();

    let worker1 = Worker {
        name: String::from("worker_asd"),
        id: String::from("1324qw"),
        age: TOTAL_COUNT + 34,
    };
    println!("========{}", worker1.name);

    let d = add;
    let c1 = d(44, 55);
    println!("c1:{}", c1);

    let d2 = mult_value(45);
    println!("{}", d2.0);
    println!("{}", d2.1);

    ///////////////////////////////
    let i: u32 = 19876;
    let p_imm: *const u32 = &i as *const u32; // explicit cast

    println!("p_imm --> {:?},{:?}", p_imm, unsafe { *p_imm });

    let mut m: u32 = 2;
    let p_mut: *mut u32 = &mut m; // implicit coercion

    unsafe {
        let _ref_imm: &u32 = &*p_imm;
        let _ref_mut: &mut u32 = &mut *p_mut;
    }

    let dog = DogSwim {
        name: "tom".to_string(),
    };
    dog.do_swim();

    let cat = CatSwim {
        name: "cat".to_string(),
    };
    cat.do_swim();
}
