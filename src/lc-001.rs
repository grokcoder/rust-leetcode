use std::collections::HashMap;

impl Solution {

    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut rs: Vec<i32> = Vec::new();
        for i in 0..nums.len() {
            for j in i +1 .. nums.len() {
                if nums[i] + nums[j] == target {
                    rs.push(i as i32);
                    rs.push(j as i32);
                    return rs
                }
            }
        }
        return rs;
    }

    // the second solution
    pub fn two_sum_s2(nums: Vec<i32>, target: i32) -> Vec<i32> {

        let mut i1 :i32 = 0;
        let mut i2 = 0;
        let mut m: HashMap<i32, i32> = HashMap::new();

        for i in 0.. nums.len() {
            let sub = target - &nums[i];
            if m.contains_key(&sub){
                i1 = i as i32;
                i2 = *m.get(&sub).unwrap();
                break;
            }else {
                m.insert(nums[i], i as i32);
            }
        }

        if i1 < i2 {
            return vec![i1, i2];
        }else {
            return vec![i2, i1];
        }
    }
}