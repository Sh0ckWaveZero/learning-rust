/// # การจัดการหน่วยความจำใน Rust
///
/// ## ภาพรวม
/// Rust จัดการหน่วยความจำผ่านสองพื้นที่หลัก: Stack และ Heap
/// ด้วยระบบการครอบครองและการยืมที่ซับซ้อน
///
/// ## Stack
/// - จัดการอัตโนมัติโดย Rust
/// - การจัดสรรหน่วยความจำที่รวดเร็ว
/// - เก็บข้อมูลขนาดคงที่
/// - เก็บเฟรมการเรียกฟังก์ชันและชนิดข้อมูลพื้นฐาน
///
/// ## Heap
/// - พื้นที่หน่วยความจำที่จัดการด้วยตนเอง
/// - การจัดสรรหน่วยความจำแบบไดนามิก
/// - เก็บข้อมูลที่มีขนาดไม่ทราบตอนคอมไพล์
/// - ต้องจัดการหน่วยความจำอย่างชัดเจน
///
/// ## References
/// - ให้การเข้าถึงข้อมูลโดยไม่เป็นเจ้าของ
/// - ใช้งานผ่านกฎการยืม
/// - ป้องกันการอ้างอิงที่ไม่ถูกต้องและการแข่งขันของข้อมูล
///
/// # ตัวอย่าง
/// ```
/// // การจองหน่วยความจำบน Stack
/// let x = 5;
///
/// // การจองหน่วยความจำบน Heap
/// let y = Box::new(10);
///
/// // การอ้างอิง
/// let z = &x;
/// ```
///
/// ## ที่อยู่หน่วยความจำ
/// - เก็บตำแหน่งหน่วยความจำ ไม่ใช่ค่าโดยตรง
/// - ตัวชี้อ้างอิงตำแหน่งหน่วยความจำเฉพาะ
///
/// # กฎการครอบครอง
/// 1. แต่ละค่ามีเจ้าของเดียว
/// 2. เมื่อเจ้าของหมดขอบเขต หน่วยความจำจะถูกปล่อยอัตโนมัติ
/// 3. ถ้ามีตัวแปรไหนที่ไม่ได้ใช้งาน หน่วยความจำจะถูกปล่อยอัตโนมัติ dropped โดย Rust
///
/// ตัวอย่างต่อไปนี้จะแสดงการใช้งานของ References ใน Rust
pub fn run() {
    // การจองหน่วยความจำบน Stack
    let x = 5;

    // การจองหน่วยความจำบน Heap
    let y = Box::new(10);

    // การอ้างอิง
    let z = &x;

    println!("x: {}", x);
    println!("y: {}", y);
    println!("z: {}", z);

    // การโอนสิทธิ์เจ้าของไปยังอีกตัวแปรหนึ่ง
    let a = Box::new(15);
    let b = a;
    // println!("a: {}", a); // ไม่สามารถใช้งานได้เพราะ a ถูกโอนสิทธิ์ไปแล้ว

    // การคัดลอกข้อมูล โดยการ clone
    // การ clone จะสร้างข้อมูลใหม่และให้สิทธิ์เจ้าของใหม่ ไม่ใช่การอ้างอิง
    // แต่ clone จะใช้หน่วยความจำเพิ่มขึ้น และไม่ควรใช้บ่อย

    let empty = y.clone();
    println!("empty: {}", empty);
    println!("y: {}", y);
}