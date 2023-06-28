
// print A-z
pub fn az2() {
    let az_lower = 'a' ..= 'z';
    let az_upper = 'A' ..= 'Z';
    let az = [az_upper, az_lower];
    for r in az {
        for c in r {
            print!("{c}");
        }
    }
    println!("\nDone");
}