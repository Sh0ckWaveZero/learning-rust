pub fn variables() {
    // 1. การตัวแปรและการเปลี่ยนแปลงค่า (mutability)
    println!("=== Variables and Mutability ===");
    let x = 5; // โดยปกติ ตัวแปรใน Rust จะกำหนดเป็น immutable คือไม่สามารถเปลี่ยนค่าได้
    println!("x is {}", x);

    let mut y = 10; // ถ้าต้องการให้ตัวแปรเป็น mutable คือเปลี่ยนค่าได้ให้ใช้คำว่า mut หน้าตัวแปร
    println!("y is {}", y);
    y = 20;
    println!("y is now {}", y);

    // 2. พื้นฐานประเภทข้อมูล (Data Types)
    println!("\n=== Basic Data Types ===");
    // สูตรการหาขนาดของข้อมูลใน Rust เช่น i32 คือ 32 bits หรือ 4 bytes 
    // ส่วน u32 คือ unsigned 32 bits หรือ 4 bytes
    // ส่วน f64 คือ 64 bits หรือ 8 bytes
    // ส่วน char คือ 32 bits หรือ 4 bytes
    // ส่วน bool คือ 1 byte
    // Range i32 คือ [-2^(n-1), 2^(n-1) - 1] โดย n คือ 32 จะได้ [-2^31, 2^31 - 1] หรือ [-2,147,483,648, 2,147,483,647]
    let integer: i32 = 42;
    let float: f64 = 3.14;
    let boolean: bool = true;
    let character: char = '😀';
    println!(
        "integer: {}, float: {}, boolean: {}, char: {}",
        integer, float, boolean, character
    );

    // 3. Constants (ค่าคงที่)
    println!("\n=== Constants ===");
    const MAX_POINTS: u32 = 100_000;
    println!("constant MAX_POINTS = {}", MAX_POINTS);

    // 4. Shadowing (การปกปิดตัวแปร)
    println!("\n=== Shadowing ===");
    let spaces = "   "; // string
    println!("spaces is a string: '{}'", spaces);
    // สามารถปกปิดตัวแปรเดิมได้โดยใช้ let ใหม่ โดยไม่ต้องใช้ mut ในการเปลี่ยนค่า
    let spaces = spaces.len(); // number
    println!("spaces is now a number: {}", spaces);

    // 5. Scope and Shadowing (การใช้งานตัวแปรในขอบเขตของตัวแปร)
    println!("\n=== Variable Scope ===");
    let outer = 1;
    {
        let inner = 2;
        println!("inner: {}, outer: {}", inner, outer);
    }
    println!("outer: {}", outer);
    // println!("inner: {}", inner); // นี้จะทำให้เกิด error เพราะ inner ไม่ได้อยู่ในขอบเขตของตัวแปร
}
