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

fn array_to_space_seperated_string(array: Vec<i32>) -> String {
    let length = array.len();
    let mut answer = array[0].to_string().to_owned();
    answer.push_str(&" ".to_owned());
    for instance in 1..length-1 {
        answer.push_str(&array[instance].to_string().to_owned());
        answer.push_str(&" ".to_owned());
    } 
    answer.push_str(&array[length-1].to_string().to_owned());
    return answer.to_string();
}


fn binary_search(array: Vec<i32>, element: i32, length: i32) -> i32 {
    let mut left = 0;
    let mut right = length - 1;

    while left <= right {
        let a = right - left;
        let b = a / 2;
        let mid = left + b;
        if array[mid as usize] == element{
            return mid;
        }
        else if element < array[mid as usize] {
            right = mid - 1;
        }
        else {
            left = mid + 1;
        }
    }
    return -1;
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
    println!("{}", array_to_space_seperated_string(vec1));
}