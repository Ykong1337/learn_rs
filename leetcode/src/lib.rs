#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let vec = vec![7, 1, 5, 3, 6, 4];
        println!("{:?}", max_profit(vec));

        let nums = vec![3, 2, 3];
        println!("{:?}", majority_element(nums));
    }



    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mid: i32 = (nums.len() / 2) as i32;

        let mut map: HashMap<i32, i32> = HashMap::new();

        for i in &nums {
            let count = map.entry(*i).or_insert(0);
            *count += 1;
        }

        for (k, v) in &map {
            if *v > mid {
                return *k;
            }
        }
        return 0;
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut min = i32::MAX;
        for p in prices {
            min = min.min(p);
            max = max.max(p - min);
        }
        max
    }

    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut vec = Vec::new();
        if nums.len() == 1 {
            vec![]
        } else {
            let mut map = HashMap::new();

            for i in &nums {
                let count = map.entry(i).or_insert(0);
                *count += 1;
            }

            for (k, v) in map {
                if v == 2 {
                    vec.push(*k);
                }
            }
            vec
        }
    }
}
