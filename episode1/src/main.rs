struct TwoInt32Fields {
    field1: i32,
    field2: i32,
}

impl TwoInt32Fields {
    fn new(field1: i32, field2: i32) -> Self {
        Self { field1, field2 }
    }
}

fn sum_by_move(tintf: TwoInt32Fields) -> i64 {
    tintf.field1 as i64 + tintf.field2 as i64
}

fn sum_by_borrow(tintf: &TwoInt32Fields) -> i64 {
    tintf.field1 as i64 + tintf.field2 as i64
}

fn main() {
    let tintf = TwoInt32Fields::new(32i32, 33i32);
    let sum1 = sum_by_borrow(&tintf);
    let sum2 = sum_by_move(tintf);
    println!("Sum: {}", sum1); 
    println!("Sum: {}", sum2); 
}
