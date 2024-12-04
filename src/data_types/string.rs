pub fn run() {
    // String ปกติ เราจะเป็น owner ของข้อมูลเสมอ และจะเก็ยข้อมูลใน heap
    // &str หรือชื่อเต็ม String slice จะเป็น reference ของ String ที่อยู่ใน heap ที่เป็นค่า mutable
    println!("=== String and &str ===");
    let map = String::from("Old map");

    // map จะถูก move ไปยัง borrow_map แต่ borrow_map จะเป็น reference ของ map
    let borrow_map = map.as_str();

    let mut crabby_map = borrow_map.to_string();

    crabby_map.push_str(" New map");
    println!("crabby_map is {}", crabby_map);
}
