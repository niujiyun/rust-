pub fn init() {
    let result = is_palindrome(1212333);
    println!("{}", result);
}
///回文数
fn is_palindrome(x: i32) -> bool {
    let s = x.to_string();
    let rev = x.to_string().chars().rev().collect::<String>();
    rev == s
}
