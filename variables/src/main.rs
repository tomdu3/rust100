fn main() {
    let mut something = 4;
    const SOMETHING: i32 = 5; // const is always immutable, type must be annotated
    let something_else = 6; // something_else is immutable
    // something_else = 7; // error: cannot assign twice to immutable variable
    println!("something is mutable: {}", something);
    println!("SOMETHING is a constant: {}", SOMETHING);
    println!("something_else is: {}", something_else);

}
  
