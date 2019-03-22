fn main() {
    let a: i32 = 4;
    if a & 1 == 1 {
        println!("Odd")
    } else {
        println!("Even")
    };

    let mut n = 0x000000001;
    for i in 1..10 {
        n = n << 3;
        println!("{}", n);
    }

    let b = 3;
    println!("{}",(!b+0x01));

    let c: String = "0123456789 - Hello bello".to_string();
    println!("{:?}",c.as_bytes());

    println!("Wohoo");
    println!("\rGrrr");
}
