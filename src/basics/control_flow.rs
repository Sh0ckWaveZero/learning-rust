pub fn control_flow() {
    // 1. if-else ใช้ในกรณีที่การตัดสินใจตามเงื่อนไข
    println!("\n=== if-else ===");
    let crabby_energy = 55;

    if crabby_energy > 75 {
        println!("The crabby is full of energy!🔋");
    } else if crabby_energy > 50 {
        println!("The crabby is feeling okay.😐");
    } else {
        println!("The crabby is feeling tired.😴");
    }

    // 2. match (pattern matching) ใช้เหมือนกับ switch-case ในภาษาอื่น ๆ
    // ใช่้ในกรณีที่ต้องการตรวจสอบค่าที่มีหลายเงื่อนไข
    print!("=== match ===\n");
    let crabby_mood = "happy";

    match crabby_mood {
        "happy" => println!("The crabby is smiling.😊"),
        "sad" => println!("The crabby is crying.😢"),
        "angry" => println!("The crabby is shouting.😡"),
        // ใช้ _ เพื่อระบุว่าเป็น case ที่ไม่ได้ระบุไว้
        _ => println!("The crabby is feeling neutral.😐"),
    }

    // 3. loop ใช้ในกรณีที่ต้องการทำงานวนไปเรื่อย ๆ โดยไม่มีเงื่อนไขใด ๆ ในการหยุด
    println!("\n=== loop ===");

    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 5 {
            break;
        }
    }
}
