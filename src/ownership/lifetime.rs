/// การใช้งาน Lifetime ใน Rust
///
/// โดยมีกฎพื้นฐาน 3 ข้อของ Lifetime:
///
/// 1.ทุก reference parameter จะได้รับ lifetime parameter โดยอัตโนมัติ
/// 2.ถ้ามี input parameter เดียว จะกำหนด lifetime ให้ output อัตโนมัติ
/// 3.ถ้าเป็น method มี &self หรือ &mut self จะกำหนด lifetime ให้ทุก parameter อัตโนมัติ
///
/// ตัวอย่างต่อไปนี้จะแสดงการใช้งาน Lifetime ใน Rust
/// Lifetime syntax ใช้ ' (apostrophe) ตามด้วยชื่อเช่น <'a>
/// โดย Lifetime จะถูกกำหนดใน <> หลังชื่อของฟังก์ชัน
/// และใช้ & ในการระบุช่วงเวลาที่ตัวแปรสามารถใช้งานได้
fn longest_map<'a>(map1: &'a str, map2: &'a str) -> &'a str {
    if map1.len() > map2.len() {
        map1
    } else {
        map2
    }
}

pub fn run() {
    // let treasure;
    // {
    //     let local_treasure = String::from("💰");
    //     treasure = &local_treasure;
    // }
    // println!("Treasure: {}", treasure); // ไม่สามารถใช้งานได้ เนื่องจาก local_treasure ถูกยกเลิกแล้ว

    let map1 = String::from("🗺️🗺️");
    let map2 = String::from("📍");

    let chosen_map = longest_map(&map1, &map2);
    println!("Crabby 's longest map: {}", chosen_map);
}
