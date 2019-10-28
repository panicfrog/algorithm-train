mod two_sum;
mod add_two_numbers;
mod length_of_longest_substring;

fn main() {
    let r = two_sum::two_sum(vec![1, 2, 3, 4], 3);
    let l = length_of_longest_substring::length_of_longest_substring(String::from("aabaab!bb"));
    println!("{:?}", r);
    println!("{:?}", l);
}
