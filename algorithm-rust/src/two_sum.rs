use std::collections::HashMap;
#[allow(dead_code)]
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut m: HashMap<i32, usize> = HashMap::new();
    for (i, v) in nums.iter().enumerate() {
        let remind = target - *v;
        if let Some(si) = m.get(&remind) {
            return  vec![*si as i32, i as i32];
        }
        m.insert(*v, i);
    }
    vec![]
}