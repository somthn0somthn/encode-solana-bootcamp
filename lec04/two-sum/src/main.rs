use two_sum::two_sum;

fn main() {
    let nums: Vec<i32> = vec![3,3];
    let target: i32 = 6;
    print!("{:?}\n", two_sum(nums, target))
}