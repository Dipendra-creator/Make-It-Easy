fn str_input() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    return line.to_string();
}

fn int_input() -> i32 {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    let n: i32 = line.trim().parse().expect("invalid input");
    return n;
}

fn int_space_input() -> Vec<i32> {
    let mut vec0 = Vec::new();
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).expect("input");
    let nums = line.trim().split(' ').flat_map(str::parse::<i32>).collect::<Vec<_>>();
    for num in nums {
        vec0.push(num);
    }
    return vec0;
}

fn main() {
    println!("Input Your Name: ");
    let str_value: String = str_input();
    println!("Output Of str_input: {}", str_value);
    println!("Input Int Values: ");
    let int_value1: i32 = int_input();
    let int_value2: i32 = int_input();
    println!("Output Of int_input by adding two values = {}", int_value1 + int_value2);
    println!("Space Seprated List Input Function");
    let vec1 = int_space_input();
    println!("{:?}", vec1);
}