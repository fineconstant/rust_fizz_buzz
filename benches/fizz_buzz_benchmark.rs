use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use fizz_buzz::FizzBuzz;

fn fizz_buzz_compare_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("FizzBuzz compare methods");

    let args = [1, 3, 5, 15];

    for arg in args.iter() {
        group.throughput(Throughput::Elements(args.len() as u64));
        group.bench_with_input(
            BenchmarkId::new("FizzBuzz::for_number", arg),
            arg,
            |bencher, &arg| bencher.iter(|| FizzBuzz::for_number(arg)),
        );
    }

    group.finish()
}

fn fizz_buzz_single_benchmark(c: &mut Criterion) {
    c.bench_function("FizzBuzz::for_number(1)", |b| {
        b.iter(|| FizzBuzz::for_number(black_box(1)))
    });

    c.bench_function("FizzBuzz::for_number(3)", |b| {
        b.iter(|| FizzBuzz::for_number(black_box(3)))
    });

    c.bench_function("FizzBuzz::for_number(5)", |b| {
        b.iter(|| FizzBuzz::for_number(black_box(5)))
    });

    c.bench_function("FizzBuzz::for_number(15)", |b| {
        b.iter(|| FizzBuzz::for_number(black_box(15)))
    });

    c.bench_function("FizzBuzz::for_number - complex", |b| {
        b.iter(|| {
            FizzBuzz::for_number(black_box(1));
            FizzBuzz::for_number(black_box(3));
            FizzBuzz::for_number(black_box(5));
            FizzBuzz::for_number(black_box(15));
        })
    });
}

criterion_group!(
    benches,
    fizz_buzz_compare_benchmark,
    fizz_buzz_single_benchmark
);

criterion_main!(benches);
