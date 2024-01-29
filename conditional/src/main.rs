fn main() {
    // traditional conditional - ; not obligatory at the end
    let mut msg = String::new();

    let num: i32 = 4;
    if num == 5 {
        msg = "five".to_string();
    } else if num == 4 {
        msg = "four".to_string();
    }

    println!("{}",msg);

    // rust typical conditional - ; obligatory at the end
    msg = if num == 5 {
        "five".to_string()
        } else if num == 4 {
            "four".to_string()
        } else {
            "other".to_string()
        };

    println!("{}", msg);
}
