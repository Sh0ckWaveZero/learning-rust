pub fn run() {
    // 1. การใช้ result ในการจัดการ error
    println!("\n=== result ===");

    // สร้าง function ที่ return Result โดยใช้ Result ในการจัดการ error
    fn get_result() -> Result<i32, String> {
        // สร้างตัวแปร result โดยใช้ Ok ในการกำหนดค่าที่ถูกต้อง
        let result = Ok(100);
        // สร้างตัวแปร error โดยใช้ Err ในการกำหนดค่า error
        // let result = Err("Error".to_string());
        result
    }

    // เรียกใช้ function get_result
    let result = get_result();
    // ใช้ match ในการจัดการ error ที่เกิดขึ้น
    match result {
        Ok(value) => println!("Value = {}", value),
        Err(error) => println!("Error = {}", error),
    }

    // 2. การใช้ unwrap ในการจัดการ error
    println!("\n=== unwrap ===");

    // สร้าง function ที่ return Result โดยใช้ Result ในการจัดการ error
    fn get_result_unwrap() -> Result<i32, String> {
        // สร้างตัวแปร result โดยใช้ Ok ในการกำหนดค่าที่ถูกต้อง
        let result = Ok(100);
        // สร้างตัวแปร error โดยใช้ Err ในการกำหนดค่า error
        // let result = Err("Error".to_string());
        result
    }

    // เรียกใช้ function get_result_unwrap และใช้ unwrap ในการจัดการ error
    let value = get_result_unwrap().unwrap();
    println!("Value = {}", value);

    // 3. การใช้ expect ในการจัดการ error
    println!("\n=== expect ===");

    // สร้าง function ที่ return Result โดยใช้ Result ในการจัดการ error
    fn get_result_expect() -> Result<i32, String> {
        // สร้างตัวแปร result โดยใช้ Ok ในการกำหนดค่าที่ถูกต้อง
        let result = Ok(100);
        // สร้างตัวแปร error โดยใช้ Err ในการกำหนดค่า error
        // let result = Err("Error".to_string());
        result
    }

    // เรียกใช้ function get_result_expect และใช้ expect ในการจัดการ error
    let value = get_result_expect().expect("Error");
    println!("Value = {}", value);
}
