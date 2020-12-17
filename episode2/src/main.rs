use std::time::SystemTime;

// Compare the time it takes to execute 1000000000 iterations
// of a simple loop using an iterator (for) and counter (while).
// Increment a counter at each iteration and print the result
// after the loop concludes in order to keep the compiler from
// optimizing the loop into oblivion.

fn main() {
    let mut i = 0;

    // First, the loop using an iterator.
    let mut before = SystemTime::now();
    for _ in 0..1000000000 {
        i += 1;
    }
    let mut after = SystemTime::now();

    print!("i: {}, ", i);

    let duration_iterator = after.duration_since(before);

    // Reset i.
    i = 0;

    // Second, the loop using a counter.
    before = SystemTime::now();
    while i < 1000000000 {
        i += 1;
    }
    after = SystemTime::now();

    print!("i: {}, ", i);

    let duration_while = after.duration_since(before);

    // Log in a CSV style for easy parsing.
    println!("{:?}, {:?}", duration_iterator, duration_while);
}
