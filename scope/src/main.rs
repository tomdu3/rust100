fn main() {
    let x = 5;  // x is immutable
    {
        let x = 99;  // x is immutable, but different scope
        println!("{}", x);
    }

    println!("{}", x);

    // redefine variables
    let mut x = 100; // x is now mutable


}
