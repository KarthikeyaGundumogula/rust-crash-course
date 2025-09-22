pub fn sum(nums: Vec<i32>) -> i32 {
    let mut sum:i32 = 0;
    for i in nums {
        sum+= i;
    }
    sum
}

pub fn fill(i: u32, n: usize) -> Vec<u32> {
    // return vec![i;n]
    let mut v = vec![];
    let mut a = 0;
    while a<n{
        v.push(i);
        a+=1;
    }
    v
}
