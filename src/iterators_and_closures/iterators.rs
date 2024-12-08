pub fn run() {
    // การใช้ iterator ในการทำงานวนไปเรื่อย ๆ ตามเงื่อนไขที่กำหนด
    println!("\n=== iterator ===");

    let treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการวนลูปเพื่อแสดงข้อมูลใน treasures ทีละตัว
    for treasure in treasures.iter() {
        println!("treasure = {}", treasure);
    }
}
