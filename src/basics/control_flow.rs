pub fn control_flow() {
    // 1. if-else
    println!("\n=== if-else ===");
    let crabby_energy = 55;

    if crabby_energy > 75 {
        println!("The crabby is full of energy!🔋");
    } else if crabby_energy > 50 {
        println!("The crabby is feeling okay.😐");
    } else {
        println!("The crabby is feeling tired.😴");
    }
}
