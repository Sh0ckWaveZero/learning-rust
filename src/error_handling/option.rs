pub fn run() {
    // 1. การใช้ option ในการจัดการ error
    println!("\n=== option ===");

    // สร้าง function ที่ return Option โดยใช้ Option ในการจัดการ error
    fn get_option(is_empty: bool) -> Option<i32> {
        if is_empty {
            Some(100)
        } else {
            None
        }
    }

    // เรียกใช้ function get_option
    let result = get_option(false);
    // ใช้ match ในการจัดการ error ที่เกิดขึ้น
    match result {
        Some(value) => println!("Value = {}", value),
        None => println!("Error"),
    }

    // 2. การใช้ unwrap ในการจัดการ error
    println!("\n=== unwrap ===");

    // สร้าง function ที่ return Option โดยใช้ Option ในการจัดการ error
    fn get_option_unwrap(is_empty: bool) -> Option<i32> {
        if is_empty {
            Some(100)
        } else {
            None
        }
    }
    // เรียกใช้ function get_option_unwrap และใช้ match ในการจัดการ error
    match get_option_unwrap(false) {
        Some(value) => println!("Value = {}", value),
        None => println!("No value found"),
    }

    match get_option_unwrap(true) {
        Some(value) => println!("Value = {}", value),
        None => println!("No value found"),
    }
}
