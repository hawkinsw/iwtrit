use std::time::SystemTime;

fn main() {

    let mut before = SystemTime::now();
    let mut i: i32 = 0; 
    for _ in 0..1000000000 {
    //while i<1000000000 {
        i+=1;
    }
    let mut after = SystemTime::now();
    println!("Run time for borrow: {:?}", after.duration_since(before));

    before = SystemTime::now();
    i =0;
    for _ in 0..1000000000 {
    //while i<1000000000 {
        i+=1;
    }
    after = SystemTime::now();
    println!("Run time for borrow: {:?}", after.duration_since(before));
}
