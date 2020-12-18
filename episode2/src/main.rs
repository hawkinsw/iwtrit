use std::time::SystemTime;
use volatile::Volatile;


// Compare the time it takes to execute 1000000000 iterations
// of a simple loop using an iterator (for) and counter (while).
// Use a volatile wrapper around the work done in the loop body
// so that the compiler does not optimize the loop into oblivion.
fn main() {
    let mut j = 0;
    let mut v = Volatile::new(&mut j);

    // Call SystemTime::now() to warm up the code path; not doing so
    // may bias the results toward the second version of the loop.
    #[allow(unused_assignments)]
    let mut before = SystemTime::now();
    #[allow(unused_assignments)]
    let mut after = SystemTime::now();

    // First, the loop using an iterator.
    before = SystemTime::now();
    for _ in 0..1000000000 {
        v.write(v.read() + 1);
    }
    after = SystemTime::now();

    let duration_iterator = after.duration_since(before);

    // Reset j.
    v.write(0);

    // Second, the loop using a counter.
    let mut i = 0;
    before = SystemTime::now();
    while i < 1000000000 {
        v.write(v.read() + 1);
        i += 1;
    }
    after = SystemTime::now();

    let duration_while = after.duration_since(before);

    // Log in a CSV style for easy parsing.
    println!("{:?}, {:?}", duration_iterator, duration_while);
}
