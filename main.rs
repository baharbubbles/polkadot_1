fn concatenate_strings(string1: &str, string2: &str) -> String {
    let mut result = String::new();
    result.push_str(string1);
    result.push_str(string2);
    result
}

fn main() {
    let string1 = "Hello, ";
    let string2 = "world!";
    
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("{}", concatenated_string);
}
