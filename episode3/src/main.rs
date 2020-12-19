use volatile::Volatile;

fn main() {
    let mut j = 0;
    let mut v = Volatile::new(&mut j);

    for _ in 0..1000000000 {
        v.write(v.read() + 1);
    }
}
