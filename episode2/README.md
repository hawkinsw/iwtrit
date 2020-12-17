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

That got us thinking, are there any performance benefits to using the `for` or the `while`?

## The Raw Materials

We wrote Rust code to investigate the performance differences. You can see the documented source code in `src/main.rs`. In order to generate data on your system, run `make csv` (assuming that you are on a `make`-capable system and have the necessary tools installed).

This generates a file named `csv.csv` that can be plotted. We recommend LibreOffice's Calc.

## Discussion

TBD
