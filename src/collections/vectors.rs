pub fn run() {
    println!("=== Vectors ===");

    // 1. สร้าง Vector ที่เก็บข้อมูลชนิด String
    // ส่วนประกอบของ Vector คือ
    // - ข้อมูลที่เก็บจะเป็นชนิดเดียวกัน
    // - ขนาดของ Vector สามารถเปลี่ยนแปลงได้
    // - สามารถเข้าถึงข้อมูลใน Vector ได้ด้วย index
    // - Vec::new() คือ constructor ของ Vector ที่จะจองพื้นที่ในหน่วยความจำในส่วนของ heap ให้กับ Vector นั้น
    let mut treasures: Vec<String> = Vec::new();

    // 2. เพิ่มข้อมูลเข้าไปใน Vector
    // {:?} ใช้ในการแสดงข้อมูลในรูปแบบของ Debug เพราะ Vector ไม่มีการ implement ของ Display
    // โดยใช้ push() เพื่อเพิ่มข้อมูลเข้าไปใน Vector โดยจะเพิ่ม capacity ทีละ 2 เท่าของข้อมูลที่เพิ่มเข้าไป
    treasures.push(String::from("🏆"));
    treasures.push(String::from("🗝️"));
    treasures.push(String::from("💰"));
    treasures.push(String::from("🔮"));
    println!("Treasures length: {}", treasures.len());
    println!("Treasures capacity: {}", treasures.capacity());
    treasures.push(String::from("💎"));
    println!("Treasures: {:?}", treasures);
    println!("Treasures length: {}", treasures.len());
    println!("Treasures capacity: {}", treasures.capacity());


    // 3. การเข้าถึงข้อมูลใน Vector
    // โดยใช้ index ในการเข้าถึงข้อมูล
    let first = &treasures[0];
    println!("The first element is: {}", first);

    // 4. การเข้าถึงข้อมูลใน Vector
    // โดยใช้ get() ในการเข้าถึงข้อมูล
    // ถ้า index ที่เราเลือกไม่มีข้อมูล จะ return None
    match treasures.get(1) {
        Some(second) => println!("The second element is: {}", second),
        None => println!("There is no second element."),
    }

    // 5. การ loop ข้อมูลใน Vector
    // ใช้ & ในการ loop ข้อมูลใน Vector โดยไม่เปลี่ยนแปลงข้อมูล
    for item in &treasures {
        println!("{}", item);
    }

    // 6. การ loop และแก้ไขข้อมูลใน Vector
    // ใช้ &mut ในการ loop และแก้ไขข้อมูลใน Vector
    for item in &mut treasures {
        item.push_str(" =💰");
    }

    // 7. การ loop ข้อมูลใน Vector หลังจากแก้ไขข้อมูล
    for item in &treasures {
        println!("{}", item);
    }

    // Note การใช้ Vector ใน Rust
    // .iter() ใช้ loop ข้อมูลใน Vector โดยไม่เปลี่ยนแปลงข้อมูล
    // .into_iter() ใช้ loop ข้อมูลใน Vector โดยเปลี่ยนแปลงข้อมูล
    // .iter_mut() ใช้ loop และแก้ไขข้อมูลใน Vector

    // ตัวอย่างการใช้งาน .iter()
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter() {
        println!("Number .iter: {}", number);
    }

    // ตัวอย่างการใช้งาน .into_iter()
    let numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.into_iter() {
        println!("Number into_iter: {}", number);
    }

    // ตัวอย่างการใช้งาน .iter_mut()
    let mut numbers = vec![1, 2, 3, 4, 5];
    for number in numbers.iter_mut() {
        *number += 10;
        println!("Number iter_mut: {}", number);
    }

    // 8. ใช้งาน pop() เพื่อลบข้อมูลตัวสุดท้ายออกจาก Vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    let last = numbers.pop();
    println!("Last number: {:?}", last);

    // 9. ใช้งาน remove() เพื่อลบข้อมูลตาม index ที่เรากำหนด
    let mut numbers = vec![1, 2, 3, 4, 5];
    let second = numbers.remove(1);
    println!("Second number removed: {:?}", second);

    // 10. ใช้งาน insert() เพื่อเพิ่มข้อมูลใน index ที่เรากำหนด
    let mut numbers = vec![1, 2, 3, 4, 5];
    numbers.insert(1, 10);
    println!("Numbers with 10 inserted: {:?}", numbers);

    // 11. ใช้งาน split_at() เพื่อแบ่ง Vector ออกเป็น 2 ส่วน
    let numbers = vec![1, 2, 3, 4, 5];
    let (left, right) = numbers.split_at(2);
    println!("Left: {:?}", left);
    println!("Right: {:?}", right);

    // 12. ใช้งาน append() เพื่อเพิ่มข้อมูลจาก Vector อีกตัวหนึ่ง
    let mut numbers = vec![1, 2, 3];
    let mut other_numbers = vec![4, 5, 6];
    numbers.append(&mut other_numbers);
    println!("Numbers after append: {:?}", numbers);

    // 13. ใช้งาน extend() เพื่อเพิ่มข้อมูลจาก Iterator อีกตัวหนึ่ง
    let mut numbers = vec![1, 2, 3];
    let other_numbers = vec![4, 5, 6];
    numbers.extend(other_numbers);
    println!("Numbers after extend: {:?}", numbers);

    // 14. ใช้งาน clear() เพื่อลบข้อมูลทั้งหมดใน Vector
    let mut numbers = vec![1, 2, 3];
    println!("Numbers before clear: {:?}", numbers);
    numbers.clear();
    println!("Numbers after clear: {:?}", numbers);

    // 15. ใช้งาน drain() เพื่อลบข้อมูลใน Vector และ return Iterator ของข้อมูลที่ถูกลบ
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before drain: {:?}", numbers);
    let mut iter = numbers.drain(1..4);
    println!("Drain 1..4: {:?}", iter.next());

    // 16. ใช้งาน retain() เพื่อลบข้อมูลใน Vector ตามเงื่อนไขที่เรากำหนด
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before retain: {:?}", numbers);
    numbers.retain(|&x| x % 2 == 0);
    println!("Numbers after retain: {:?}", numbers);

    // 17. ใช้งาน resize() เพื่อเปลี่ยนขนาดของ Vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before resize: {:?}", numbers);
    numbers.resize(3, 0);
    println!("Numbers after resize: {:?}", numbers);

    // 18. ใช้งาน reverse() เพื่อกลับลำดับของข้อมูลใน Vector
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before reverse: {:?}", numbers);
    numbers.reverse();
    println!("Numbers after reverse: {:?}", numbers);

    // 19. ใช้งาน sort() เพื่อเรียงลำดับข้อมูลใน Vector
    let mut numbers = vec![5, 3, 1, 2, 4];
    println!("Numbers before sort: {:?}", numbers);
    numbers.sort();
    println!("Numbers after sort: {:?}", numbers);

    // 20. ใช้งาน sort_by() เพื่อเรียงลำดับข้อมูลใน Vector ตามเงื่อนไขที่เรากำหนด
    let mut numbers = vec![5, 3, 1, 2, 4];
    println!("Numbers before sort_by: {:?}", numbers);
    numbers.sort_by(|a, b| b.cmp(a));
    println!("Numbers after sort_by: {:?}", numbers);

    // 21. ใช้งาน sort_by_key() เพื่อเรียงลำดับข้อมูลใน Vector ตาม key ที่เรากำหนด
    let mut numbers = vec![5, 3, 1, 2, 4];
    println!("Numbers before sort_by_key: {:?}", numbers);
    numbers.sort_by_key(|a| a.to_string());
    println!("Numbers after sort_by_key: {:?}", numbers);

    // 22. ใช้งาน sort_by_cached_key() เพื่อเรียงลำดับข้อมูลใน Vector ตาม key ที่เรากำหนด
    let mut numbers = vec![5, 3, 1, 2, 4];
    println!("Numbers before sort_by_cached_key: {:?}", numbers);
    numbers.sort_by_cached_key(|a| a.to_string());
    println!("Numbers after sort_by_cached_key: {:?}", numbers);

    // 23. ใช้งาน split_off() เพื่อแบ่ง Vector ออกเป็น 2 ส่วน
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before split_off: {:?}", numbers);
    let other_numbers = numbers.split_off(2);
    println!("Numbers after split_off: {:?}", numbers);
    println!("Other numbers: {:?}", other_numbers);

    // 24. ใช้งาน swap() เพื่อสลับข้อมูลใน Vector ตาม index ที่เรากำหนด
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before swap: {:?}", numbers);
    numbers.swap(1, 3);
    println!("Numbers after swap: {:?}", numbers);

    // 25. ใช้งาน truncate() เพื่อลบข้อมูลใน Vector ตั้งแต่ index ที่เรากำหนดไปจนจบ
    let mut numbers = vec![1, 2, 3, 4, 5];
    println!("Numbers before truncate: {:?}", numbers);
    numbers.truncate(2);
    println!("Numbers after truncate: {:?}", numbers);

    // 26. ใช้งาน dedup() เพื่อลบข้อมูลที่ซ้ำกันใน Vector
    let mut numbers = vec![1, 2, 2, 3, 3, 4, 5];
    println!("Numbers before dedup: {:?}", numbers);
    numbers.dedup();
    println!("Numbers after dedup: {:?}", numbers);

    // 27. ใช้งาน dedup_by() เพื่อลบข้อมูลที่ซ้ำกันใน Vector ตามเงื่อนไขที่เรากำหนด
    let mut numbers = vec![1, 2, 2, 3, 3, 4, 5];
    println!("Numbers before dedup_by: {:?}", numbers);
    numbers.dedup_by(|a, b| a == b);
    println!("Numbers after dedup_by: {:?}", numbers);

    // 28. ใช้งาน dedup_by_key() เพื่อลบข้อมูลที่ซ้ำกันใน Vector ตาม key ที่เรากำหนด
    let mut numbers = vec![1, 2, 2, 3, 3, 4, 5];
    println!("Numbers before dedup_by_key: {:?}", numbers);
    numbers.dedup_by_key(|a| *a);
    println!("Numbers after dedup_by_key: {:?}", numbers);

    // 29. ใช้งาน dedup_by_key() เพื่อลบข้อมูลที่ซ้ำกันใน Vector ตาม key ที่เรากำหนด
    let mut numbers = vec![1, 2, 2, 3, 3, 4, 5];
    println!("Numbers before dedup_by_key: {:?}", numbers);
    numbers.dedup_by_key(|a| *a);
    println!("Numbers after dedup_by_key: {:?}", numbers);
}
