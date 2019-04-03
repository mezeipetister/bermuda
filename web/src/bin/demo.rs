// Tests

pub fn transform_string() -> String {
    let mut a: String = "Hello {name}!".to_string();
    println!("{}", a.replace("{name}", "Peti"));
}

fn main() {
    transform_string();
}
