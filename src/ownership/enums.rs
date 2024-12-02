enum CrabbyState {
    Fighting,
    Collocating(u8),
    Defending,
}

impl CrabbyState {
    // การประกาศ method ให้กับ enum ใช้คำสั่ง impl ตามด้วยชื่อ enum และ method ที่ต้องการ
    fn take_damage(&mut self, damage: u8) {
        // ใช้ match เพื่อเข้าถึงแต่ละ variant ของ enum
        // โดยใช้ self ในการเข้าถึง variant ของ enum
        match self {
            CrabbyState::Fighting => {
                println!("Crabby is fighting!🦀");
            }
            CrabbyState::Collocating(hit_points) => {
                // * ใช้เพื่อ dereference ค่าใน enum ที่เป็น reference
                // โดยในกรณีนี้เป็น u8 ที่เป็น hit_points
                *hit_points = hit_points.saturating_sub(damage);
                println!("Crabby is collocating!🦀 {}", hit_points);
            }
            CrabbyState::Defending => {
                println!("Crabby is defending!🦀");
            }
        }
    }
}

pub fn run() {
    let mut fighting = CrabbyState::Fighting;
    let mut collocating = CrabbyState::Collocating(100);
    let mut defending = CrabbyState::Defending;

    println!("=== Enums ===");
    fighting.take_damage(10);
    collocating.take_damage(10);
    defending.take_damage(10);
}
