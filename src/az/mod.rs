pub mod az2;

// print a-Z
pub fn az1() {
    let az_lower = 'a' ..= 'z';
    let az_upper = 'A' ..= 'Z';
    let az = [az_lower, az_upper];
    for r in az {
        for c in r {
            print!("{c}");
        }
    }
    println!("\nDone");
}