pub fn run() {
    // 1. การใช้ loop ในการทำงานวนไปเรื่อย ๆ โดยไม่มีเงื่อนไขใด ๆ ในการหยุด
    println!("\n=== loop ===");

    let mut counter = 0;
    loop {
        counter += 1;
        println!("counter = {}", counter);

        if counter == 5 {
            break;
        }
    }

    // 2. การใช้ while ในการทำงานวนไปเรื่อย ๆ ตามเงื่อนไขที่กำหนด
    println!("\n=== while ===");

    let mut counter = 0;
    while counter < 5 {
        counter += 1;
        println!("counter = {}", counter);
    }

    // 3. การใช้ for ในการทำงานวนไปเรื่อย ๆ ตามเงื่อนไขที่กำหนด
    println!("\n=== for ===");

    // ใช้ for ในการวนลูปจาก 1 ถึง 6
    for counter in 1..6 {
        println!("counter = {}", counter);
    }

    let treasures = ["Gold", "Silver", "Diamond"];
    // ใช้ for ในการวนลูปเพื่อแสดงข้อมูลใน treasures ทีละตัว &&str คือ reference ของ String
    for treasure in treasures.iter() {
        // ข้าม loop ถ้าเจอข้อมูล Silver &"Silver" คือ reference ของ String
        if treasure == &"Silver" {
            println!("Skip Silver");
            continue;
        }
        println!("treasure = {}", treasure);
    }
}
