# Episode 2

## The Premise
The Rust book [recommends](https://doc.rust-lang.org/1.2.0/book/for-loops.html) writing loops using the iterator-based `for` construct:

```
for var in expression {
    code
}
```

The `expression` is an iterator and `code` executes once for each different value of `var` obtained from it.

Of course, Rust also supports `while` loops, too:

```
while expression {
    code
}
```

In this loop, `code` will be executed repeatedly until `expression` becomes `false`. 

In Rust, the infix [`..`](https://doc.rust-lang.org/reference/expressions/range-expr.html) can be used to generate an iterator that produces a sequence of integers. For example, `25..35` is an iterator that produces 25, 26, 27 ... 35. 

In Rust, then, you could write a loop that executes `code` 100 times in two ways:

```
for _ in 1..100 {
    code
}

let mut i = 0;
while i < 100 {
    code
    i+=1;
}
```

That got us thinking, are there any performance benefits to using the `for` or the `while`? We will refer to the `for`-style loop as an iterator-based loop and the `while` loop as a counter-based loop.

## The Raw Materials

We wrote Rust code to investigate the performance differences. You can see the documented source code in `src/main.rs` of the `episode2` folder of the IWTRIT repository. In order to generate data on your system, run `make csv` (assuming that you are on a `make`-capable system and have the necessary tools installed).

This generates a file named `csv.csv` that can be plotted. We recommend LibreOffice's Calc.

## The Research

Find out how we did the research, watch the stream [online](https://www.youtube.com/watch?v=EADrOfWwdWA).

## The Answer

There is no appreciable difference in the performance of a loop written using a counter or an iterator. This is great news. It means that Rust programmers are free to follow the best advice of the Rust documentation and write loops using iterators.

## Walkthrough

Let's walkthrough the Rust code posted for this episode. 

```
fn main() {
    let mut j = 0;
    let mut v = Volatile::new(&mut j);
```
In order to keep the compiler from optimizing the loops into oblivion, we have to tell the optimizer that meaningful work is happening in their bodies. The `Volatile` crate helps us do that. It wraps a reference to a variable and we use that wrapper to read and write the variable's values. The wrapper's code tells the optimizer that those indirect reads and writes cannot be optimized.

```
    #[allow(unused_assignments)]
    let mut before = SystemTime::now();
    #[allow(unused_assignments)]
    let mut after = SystemTime::now();
```

Because of misses in the instruction cache, the first call to `SystemTime::now()` takes longer than subsequent calls. This noise will affect our results if we are not careful. Therefore, we call `SystemTime::now()` twice in order to prime the i-cache.

```
    before = SystemTime::now();
    for _ in 0..1000000000 {
        v.write(v.read() + 1);
    }
    after = SystemTime::now();

    let duration_iterator = after.duration_since(before);

```

Time the iterator-based loop. Note that the work that we are doing inside the loop is using the `Volatile`-wrapped variable so that the optimizer keeps its hands off!

```

    // Reset j.
    v.write(0);

```
Not strictly necessary, but still nice: Reset the counter variable to 0.
```


    let mut i = 0;
    before = SystemTime::now();
    while i < 1000000000 {
        v.write(v.read() + 1);
        i += 1;
    }
    after = SystemTime::now();

    let duration_while = after.duration_since(before);
```

Time the iterator-based loop.

```
    // Log in a CSV style for easy parsing.
    println!("{:?}, {:?}", duration_iterator, duration_while);
}
```

Last, but not least, print the results to the screen so our visual inspection!
