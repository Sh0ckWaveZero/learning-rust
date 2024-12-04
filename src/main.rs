// Rust จะเริ่มทำงานจากไฟล์ src/main.rs โดยปกติ
// แต่ก็สามารถใช้ไฟล์อื่นเป็น entry point ได้ โดยใช้คำสั่ง cargo new my_project --bin

// การเรียกใช้งานไฟล์อื่น src/ basics/variables.rs
// จะต้องใช้คำสั่ง mod และ use ในไฟล์ src/main.rs

// 1. ใช้คำสั่ง mod เพื่อเรียกใช้งานไฟล์อื่น
// mod basics {
// pub mod control_flow;
// pub mod functions;
// pub mod variables;
// }

// mod ownership {
//     // pub mod references;
//     // pub mod borrowing;
//     // pub mod lifetime;
// }

// mod structs_basic {
//     pub mod structs;
// }

// mod traits_and_generics {
//     pub mod basic_traits;
// }

// mod data_types {
//     pub mod string;
// }

mod flow {
    pub mod basic_loop;
}

// 2. ใช้คำสั่ง use เพื่อเรียกใช้งานไฟล์อื่น
// use basics::control_flow
// use basics::variables;
// use basics::functions;

// use ownership::references;
// use ownership::borrowing;
// use ownership::lifetime;
// use ownership::structs;

// use structs_basic::structs;

// use traits_and_generics::basic_traits;

// use data_types::string;

use flow::basic_loop;

fn main() {
    // println!("Hello, Crabby! 🦀");
    // variables::variables();
    // control_flow::control_flow();
    // let item = "🦀";
    // let result = functions::hello(item);
    // println!("{}", result);

    // borrowing::run();
    // lifetime::run();

    // structs::run();
    // basic_traits::run();

    // string::run();

    basic_loop::run();
}
