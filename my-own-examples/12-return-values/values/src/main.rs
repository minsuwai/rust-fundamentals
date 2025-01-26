
fn split_string(s: String, delimiter: char, field: usize) -> String {
    let parts: Vec<&str> = s.split(delimiter).collect();
    // This will not compile!
    let result = parts.get(field);
    result.unwrap().to_string()
}

fn main() {
    let chunk = split_string("min,su,wai".to_string(), ',', 2);
    println!("Split string: {}", chunk);
}
