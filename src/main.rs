// Rust จะเริ่มทำงานจากไฟล์ src/main.rs โดยปกติ
// แต่ก็สามารถใช้ไฟล์อื่นเป็น entry point ได้ โดยใช้คำสั่ง cargo new my_project --bin

// การเรียกใช้งานไฟล์อื่น src/ 01-basics/variables.rs
// จะต้องใช้คำสั่ง mod และ use ในไฟล์ src/main.rs

// 1. ใช้คำสั่ง mod เพื่อเรียกใช้งานไฟล์อื่น
mod basics {
    pub mod control_flow;
    // pub mod variables;
}

// 2. ใช้คำสั่ง use เพื่อเรียกใช้งานไฟล์อื่น
use basics::control_flow;
// use basics::variables;

fn main() {
    println!("Hello, Crabby! 🦀");
    // variables::variables();
    control_flow::control_flow();
}
