#[allow(dead_code)]

pub fn input_signed_int(mut input: String) -> i32{
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let return_var = input.trim().parse().expect("Input not an integer");

    return return_var;
}

pub fn input_unsigned_int(mut input: String) -> u32{
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let return_var: u32 = input.trim().parse().expect("Invalid input: ((this function only accepts unsigned ints))");

    return return_var;
}

pub fn input_float(mut input: String) -> f32{
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let return_var: f32 = input.trim().parse().expect("Input not a float");

    return return_var;
}

pub fn input_char(mut input: String) -> char{
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let return_var: char = input.trim().parse().expect("Input not a char");

    return return_var;
}

pub fn input_str(mut input: String) -> String{
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    let return_var = input.trim().to_string();

    return return_var;
}
