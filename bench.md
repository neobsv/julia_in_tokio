# Benchmark Results

## Out of the Box

## The Standard library Threads

```bash
% cargo bench
   Compiling julia v0.1.0 (/Users/bsriniva/other/julia_in_tokio/julia)
    Finished `bench` profile [optimized] target(s) in 0.39s
     Running unittests src/main.rs (target/release/deps/julia-646c7cabc7ccf38f)

running 2 tests
test tests::test_functional ... ignored
test tests::bench_generate_image ... bench:  18,998,616 ns/iter (+/- 3,099,575)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 5.71s

```

## Using Crossbeam to spawn threads

```bash
% cargo bench
   Compiling julia v0.1.0 (/Users/bsriniva/other/julia_in_tokio/julia)
    Finished `bench` profile [optimized] target(s) in 0.36s
     Running unittests src/main.rs (target/release/deps/julia-f0e0428aa51f054a)

running 2 tests
test tests::test_functional ... ignored
test tests::bench_crossbeam ... bench:  18,279,533 ns/iter (+/- 3,527,354)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 5.60s

```

## Tokio Green Threads

```bash
% cargo bench
   Compiling julia v0.1.0 (/Users/bsriniva/other/julia_in_tokio/julia)
    Finished `bench` profile [optimized] target(s) in 0.51s
     Running unittests src/main.rs (target/release/deps/julia-cf93d3e87cc58420)

running 2 tests
test tests::test_functional ... ignored
test tests::bench_tokio ... bench:   1,686,910 ns/iter (+/- 119,487)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 3.55s

```

## Using Rayon for Parallelism

```bash
% cargo bench
   Compiling julia v0.1.0 (/Users/bsriniva/other/julia_in_tokio/julia)
    Finished `bench` profile [optimized] target(s) in 0.36s
     Running unittests src/main.rs (target/release/deps/julia-3e5913b882c12dab)

running 2 tests
test tests::test_functional ... ignored
test tests::bench_rayon ... bench:     382,256 ns/iter (+/- 24,190)

test result: ok. 0 passed; 0 failed; 1 ignored; 1 measured; 0 filtered out; finished in 3.53s

```
