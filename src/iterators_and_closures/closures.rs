pub fn run() {
    // การใช้งาน closures ในรูปแบบต่าง ๆ
    println!("\n=== closures ===");

    // 1. การสร้าง closure แบบไม่มีการใช้ parameter
    let say_hello = || {
        println!("Hello, Crabby! 🦀");
    };

    // เรียกใช้ closure ที่สร้างไว้
    say_hello();

    // 2. การสร้าง closure แบบมีการใช้ parameter
    let say_hello = |name: &str| {
        println!("Hello, {}! 🦀", name);
    };

    // เรียกใช้ closure ที่สร้างไว้
    say_hello("Crabby");

    // 3. การสร้าง closure แบบมีการ return ค่า
    let add = |a: i32, b: i32| -> i32 { a + b };

    // เรียกใช้ closure ที่สร้างไว้
    let result = add(10, 20);
    println!("result = {}", result);

    // 4. การสร้าง closure แบบมีการใช้ parameter และ return ค่า
    let add = |a: i32, b: i32| -> i32 {
        let result = a + b;
        println!("{} + {} = {}", a, b, result);
        result
    };

    // เรียกใช้ closure ที่สร้างไว้
    let result = add(10, 20);
    println!("result = {}", result);

    // 5. การสร้าง closure แบบมีการใช้ parameter และ return ค่า แบบมีชื่อ parameter
    let add = |a: i32, b: i32| -> i32 {
        let result = a + b;
        println!("{} + {} = {}", a, b, result);
        result
    };

    // เรียกใช้ closure ที่สร้างไว้
    let result = add(10, 20);
    println!("result = {}", result);

    // 6. closures กับ iterator
    let treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการวนลูปเพื่อแสดงข้อมูลใน treasures ทีละตัว
    treasures.iter().for_each(|treasure| {
        println!("treasure = {}", treasure);
    });

    // 7. การใช้ closures ในการ filter ข้อมูล
    let treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการ filter ข้อมูลที่มีค่ามากกว่า 200
    let filtered_treasures: Vec<i32> = treasures
        .iter()
        .filter(|&treasure| *treasure > 200)
        .cloned()
        .collect();

    // แสดงข้อมูลที่ filter ได้
    println!("filtered_treasures = {:?}", filtered_treasures);

    // 8. การใช้ closures ในการ map ข้อมูล
    let treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการ map ข้อมูลที่มีค่ามากกว่า 200
    let mapped_treasures: Vec<i32> = treasures.iter().map(|treasure| treasure * 2).collect();

    // แสดงข้อมูลที่ map ได้
    println!("mapped_treasures = {:?}", mapped_treasures);

    // 9. การใช้ closures ในการ reduce ข้อมูล
    let treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการ reduce ข้อมูลที่มีค่ามากกว่า 200
    let reduced_treasures: i32 = treasures.iter().fold(0, |acc, treasure| acc + treasure);

    // แสดงข้อมูลที่ reduce ได้
    println!("reduced_treasures = {:?}", reduced_treasures);

    // 10. การใช้ closures ในการ sort ข้อมูล
    let mut treasures = vec![100, 200, 300, 400];

    // ใช้ iterator ในการ sort ข้อมูล
    treasures.sort_by(|a, b| b.cmp(a));

    // แสดงข้อมูลที่ sort ได้
    println!("sorted_treasures = {:?}", treasures);
}
