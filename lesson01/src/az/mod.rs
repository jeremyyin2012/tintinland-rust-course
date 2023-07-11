pub mod az2;

// print a-Z
pub fn az1() {
    for c in ('Z' ..= 'a').rev() {
        print!("{c}");
    }
    println!("\nDone");
}