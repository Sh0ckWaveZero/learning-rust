use std::collections::HashMap;

pub fn run() {
    // การใช้งาน HashMap ใน Rust
    println!("\n=== HashMap ===");

    // สร้าง HashMap ที่มี key เป็น String และ value เป็น i32
    let mut scores: HashMap<String, i32> = HashMap::new();

    // เพิ่มข้อมูลลงใน HashMap
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // แสดงข้อมูลใน HashMap
    println!("scores = {:?}", scores);

    // การใช้ Some คือ การเช็คว่ามีข้อมูลใน key ที่ระบุหรือไม่ ถ้ามีจะแสดงข้อมูลนั้นออกมา
    // ถ้าไม่มีข้อมูลจะไม่แสดงอะไรออกมา และไม่ error
    if let Some(score) = scores.get("Blue") {
        println!("score of Blue = {}", score);
    }

    // loop ในการแสดงข้อมูลใน HashMap โดยใช้ forเป็น tuple ที่มี key และ value
    // ในการแสดงข้อมูลใน HashMap ให้ใช้ &scores แทน scores เพื่อให้เป็นการอ้างอิง
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // การใช้  unwrap_or ในการเช็คว่ามีข้อมูลใน key ที่ระบุหรือไม่ ถ้ามีจะแสดงข้อมูลนั้นออกมา
    // ถ้าไม่มีข้อมูลจะแสดงค่าที่กำหนดไว้ใน unwrap_or และไม่ error
    let score = scores.get("Green").unwrap_or(&0);
    println!("score of Green = {}", score);

    // การลบข้อมูลใน HashMap
    scores.remove("Blue");

    // แสดงข้อมูลใน HashMap
    println!("scores = {:?}", scores);
}
