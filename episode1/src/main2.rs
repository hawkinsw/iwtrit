struct MixedFields {
    field1: i32,
    field2: i64,
    field3: i32,
    field4: i64,
}

impl MixedFields {
    fn new(field1: i32, field2: i64, field3: i32, field4: i64) -> Self {
        Self { field1, field2, field3, field4 }
    }
}

fn sum_by_move(mf: MixedFields) -> i64 {
    mf.field1 as i64 + mf.field2 + mf.field3 as i64 + mf.field4
}

fn sum_by_borrow(mf: &MixedFields) -> i64 {
    mf.field1 as i64 + mf.field2 + mf.field3 as i64 + mf.field4
}

fn main() {
    let mf = MixedFields::new(32i32, 33i64, 34i32, 35i64);
    let sum1 = sum_by_borrow(&mf);
    let sum2 = sum_by_move(mf);
    println!("Sum: {}", sum1); 
    println!("Sum: {}", sum2); 
}
