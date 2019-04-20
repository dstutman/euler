fn main() {
    let mut sum = 0;
    let mut sum_of_squares = 0; 
    for i in 1..101_u32 {
        sum += i;
        sum_of_squares += i.pow(2);
    }
    println!("{}", sum.pow(2)-sum_of_squares);
}
