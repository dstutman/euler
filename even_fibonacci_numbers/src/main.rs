fn main() {
    let mut sum = 0;

    let mut previous = 1;
    let mut current = 1;
    let mut next;

    // Bad practice as not infinite but since almost
    // nothing happens after exiting, is ok
    loop {
        next = previous + current;
        if next > 4000000 { break; }
        previous = current;
        current = next;
        if current % 2 == 0 { sum += current; }
    }

    println!("{}", sum);
}
