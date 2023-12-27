use std::collections::HashMap;

pub fn init() {
    let arr = [1, 2, 3, 4, 5].to_vec();
    let result = tow_sum(arr, 9);
    println!("{:?}", result);
}
/// 1. 两数之和
fn tow_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map: HashMap<_, i32> = HashMap::with_capacity(nums.len());
    for i in 0..nums.len() {
        if let Some(k) = map.get(&(target - nums[i])) {
            if *k != i as i32 {
                return vec![*k, i as i32];
            }
        }
        map.insert(nums[i], i as i32);
    }
    vec![]
}
