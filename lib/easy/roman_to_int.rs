pub fn init() {
    let result = roman_to_int("MCMXCIV".to_string());
    println!("{}", result);
}
///罗马数字转整数
fn roman_to_int(s: String) -> i32 {
    let mut num = 0;
    let mut current = 0;
    let mut prev = 0;
    for i in s.chars() {
        println!("i:{}", i);
        match i {
            'I' => current = 1,
            'V' => current = 5,
            'X' => current = 10,
            'L' => current = 50,
            'C' => current = 100,
            'D' => current = 500,
            'M' => current = 1000,
            _ => println!("Invalid character"),
        }
        num += current;
        if current > prev {
            num -= prev * 2;
        }
        prev = current;
    }
    num
}
