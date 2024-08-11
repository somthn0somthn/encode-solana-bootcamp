use std::collections::HashMap;

pub fn two_sum(vector: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in vector.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = map.get(&complement) {
            return vec![complement_index as i32, i as i32];
        }
        map.insert(num, i);
    }
    
    Vec::new()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn long() {
        let nums = vec![2,7,11,15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0,1]);
    }

    #[test]
    fn short() {
        let nums = vec![3,2,4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1,2]);
    }

    #[test]
    fn repeate() {
        let nums = vec![3,3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0,1]);
    }
}
