/// Generic ใน Rust
/// เป็นวิธีการเขียนโค้ดที่ทำงานได้กับหลายประเภทข้อมูล
/// ใช้รูปแบบ `struct` หรือ `enum` ตามด้วย `<T>` เพื่อกำหนดประเภทข้อมูล
/// โดยที่ `T` สามารถใช้ในฟิลด์หรือเมธอดต่างๆ ได้
struct Inventory<T> {
    item: T,
}

/// Trait ใน Rust
/// เป็นการกำหนดพฤติกรรมที่ต้องการให้ประเภทข้อมูลนั้นๆ มี
trait DisplayItem {
    fn display(&self);
}

/// # การใช้งาน Trait ใน Rust
///
/// ## รายละเอียด
///
/// การนำ trait `DisplayItem` มาใช้กับ struct `Inventory<T>` ประกอบด้วย:
///
/// ### การประกาศการใช้งาน Generic
/// ```rust
/// impl<T> DisplayItem for Inventory<T>
/// where
///     T: std::fmt::Debug
/// ```
/// - `impl<T>` บอกว่าจะใช้ generic type T
/// - `DisplayItem` คือ trait ที่จะนำมาใช้
/// - `for Inventory<T>` บอกว่าใช้กับ struct Inventory
/// - `where T: std::fmt::Debug` กำหนดว่า T ต้องมี trait Debug
///
/// ### การเขียนเมธอด
/// ```rust
/// fn display(&self) {
///     println!("{:?}", self.item);
/// }
/// ```
/// - `display()` คือเมธอดที่กำหนดไว้ใน trait DisplayItem
/// - `{:?}` ใช้แสดงผลในรูปแบบ debug
/// - `self.item` เรียกใช้ item จาก struct Inventory
///
/// ### หมายเหตุ
/// - จำเป็นต้องมี trait Debug เพื่อการแสดงผล
/// - ใช้ได้กับทุกประเภทข้อมูลที่มี trait Debug
impl<T> DisplayItem for Inventory<T>
where
    T: std::fmt::Debug,
{
    fn display(&self) {
        println!("{:?}", self.item);
    }
}

/// Trait ใน Rust: วิธีการกำหนดพฤติกรรมร่วมกัน
/// เปรียบเสมือน "สัญญา" ที่กำหนดว่าประเภทข้อมูลนั้นๆ ต้องมีความสามารถอะไรบ้าง
///
/// # ลักษณะสำคัญ
/// - ใช้แทนการสืบทอดแบบ Object-Oriented Programming (OOP)
/// - กำหนดพฤติกรรมที่ใช้ร่วมกันระหว่างประเภทข้อมูล
/// - ใช้คำสั่ง `impl` ในการกำหนดการทำงาน
///
/// # ตัวอย่าง
/// ```rust
/// trait Summary {
///     fn summarize(&self) -> String;
/// }
///
/// struct Article {
///     title: String,
///     content: String,
/// }
///
/// impl Summary for Article {
///     fn summarize(&self) -> String {
///         format!("{}: {}", self.title, self.content)
///     }
/// }
/// ```
pub fn run() {
    println!("=== TRAITS AND GENERICS ===");

    let inventory = Inventory { item: "🦀" };
    inventory.display();
    let inventory = Inventory { item: 42 };
    inventory.display();
    let inventory = Inventory { item: 3.14 };
    inventory.display();
}
