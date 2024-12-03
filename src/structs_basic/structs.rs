/// Structs ใน Rust คือ โครงสร้างข้อมูลที่สามารถเก็บข้อมูลหลายๆ ประเภทไว้ด้วยกัน
/// โดยสามารถกำหนด method ให้กับ struct ได้
/// โดยการประกาศ struct ให้ใช้คำสั่ง struct ตามด้วยชื่อ struct และประกาศ field ของ struct ใน {} โดยใช้ชื่อ field ตามด้วยชนิดข้อมูล
struct Crabby {
    name: String,
    skill_level: u32,
    hit_points: u8,
}

struct Position(i32, i32);

impl Crabby {
    // การประกาศ method ให้กับ struct ใช้คำสั่ง impl ตามด้วยชื่อ struct และ method ที่ต้องการ
    // โดยใน method สามารถเข้าถึง field ของ struct ได้โดยใช้ self และ . ตามด้วยชื่อ field
    fn take_damage(&mut self, damage: u8) {
        self.hit_points = self.hit_points.saturating_sub(damage); // ใช้ saturating_sub เพื่อไม่ให้ค่าลบได้ ถ้าค่าลบจะเป็น 0
        println!("{} took {} damage!⚔️", self.name, damage);
    }

    fn healing(&mut self, healing: u8) {
        if (self.hit_points + healing) > 100 {
            self.hit_points = 100;
        } else {
            self.hit_points += healing;
        }
        println!("{} healed {} hit points!🩹", self.name, healing);
    }

    fn level_up(&mut self) {
        self.skill_level += 1;
        println!("{} leveled up!🎉", self.name);
    }
}

pub fn run() {
    let mut crabby = Crabby {
        name: String::from("Crabby"),
        skill_level: 100,
        hit_points: 100,
    };

    println!("=== Structs ===");
    println!(
        "Crabby's hit points before taking damage: {}",
        crabby.hit_points
    );
    crabby.take_damage(90);
    crabby.healing(100);
    crabby.level_up();
    println!("Crabby's name: {}", crabby.name);
    println!("Crabby's skill level: {}", crabby.skill_level);
    println!("Crabby's hit points: {}", crabby.hit_points);

    // .. ใช้เพื่อ copy ค่า field ที่เหลือจาก struct ที่มีอยู่
    // ระวัง ownership ของ field ที่เป็น reference จะถูกเปลี่ยนไป ถ้าเปลี่ยนค่าของ field นั้น
    // เช่น name ใน struct Crabby ถ้าเปลี่ยนค่า name ใน new_crabby จะเปลี่ยนค่า name ใน crabby ด้วย
    let new_crabby = Crabby {
        name: String::from("New Crabby"),
        ..crabby
    };
    println!("New Crabby's name: {}", new_crabby.name);
    println!("New Crabby's skill level: {}", new_crabby.skill_level);
    println!("New Crabby's hit points: {}", new_crabby.hit_points);

    // Tuple structs ใช้เหมือน tuple แต่ใช้ struct แทน ()
    let position = Position(10, 20);
    println!("Position: ({}, {})", position.0, position.1);
}
