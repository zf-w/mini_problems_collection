fn main() {
    println!("Hello, world!");
}
pub fn maximum_candies(candies: Vec<i32>, k: i64) -> i32 {
    let candy_sum: i32 = candies.iter().sum();
    let mut begin_num: i64 = 0;
    let mut end_num: i64 = ((candy_sum as i64) / k) + 1;

    let check_closure = |mid_num: i64| -> bool {
        let mut child_satisfied_num: i64 = 0;
        for candy_pile in candies.iter().cloned() {
            child_satisfied_num += candy_pile as i64 / mid_num;
            if child_satisfied_num >= k {
                return true;
            }
        }
        false
    };

    while begin_num < end_num {
        let mid_num = (begin_num + end_num) / 2;
        if mid_num == 0 || check_closure(mid_num) == true {
            begin_num = mid_num + 1;
        } else {
            end_num = mid_num;
        }
    }
    begin_num as i32 - 1
}
