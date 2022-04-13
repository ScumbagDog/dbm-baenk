use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use dbm_baenk::{DBM, RDBM, UDBM};

macro_rules! generate_benchmarks {
    //Name should be a &str, i.e. a string "like this", type should be the type we're testing.
    ($($name:expr, $type:ty,)*) => {
        pub fn zero_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Zero");
            for i in [20u64].iter() {
                $(
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, i| b.iter(|| {let _x:$type = DBM::zero(*i as usize);}));
                )*
            }
        }

        pub fn init_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Init");
            for i in [20u64].iter() {
                $(
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, i| b.iter(|| {let _x:$type = DBM::init(*i as usize);}));
                )*
            }
        }

        pub fn inclusion_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Inclusion");
            for i in [20u64].iter() {
                $(
                    let x:$type = DBM::zero(*i as usize);
                    let y:$type = DBM::zero(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), &(x, y), |b, (x, y)| b.iter(|| {
                        DBM::is_included_in(x, y)
                    }));
                )*
            }
        }

        pub fn satisfied_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Satisfied");
            for i in [20u64].iter() {
                $(
                    let x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), &x, |b, x| b.iter(|| {
                        DBM::is_satisfied(x, 1, 0, false, 10);
                    }));
                )*
            }
        }

        pub fn close_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Close");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::close(&mut x);
                    }));
                )*
            }
        }

        pub fn future_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Future");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::future(&mut x);
                    }));
                )*
            }
        }

        pub fn past_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Past");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::past(&mut x);
                    }));
                )*
            }
        }

        pub fn restrict_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Restrict");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::restrict(&mut x, 1, 0, false, 10);
                    }));
                )*
            }
        }

        pub fn free_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Free");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::free(&mut x, 1);
                    }));
                )*
            }
        }

        pub fn assign_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Assign");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::assign(&mut x, 1, 10);
                    }));
                )*
            }
        }

        pub fn copy_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Copy");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::copy(&mut x, 1, 2); //nb: Don't run this on DBMs with dim < 3
                    }));
                )*
            }
        }

        pub fn shift_benchmark(c: &mut Criterion) {
            let mut group = c.benchmark_group("Shift");
            for i in [20u64].iter() {
                $(
                    let mut x:$type = DBM::init(*i as usize);
                    group.bench_with_input(BenchmarkId::new($name, i), i, |b, _i| b.iter(|| {
                        DBM::shift(&mut x, 1, 10);
                    }));
                )*
            }
        }

        criterion_group!(benches,
                         zero_benchmark,
                         init_benchmark,
                         inclusion_benchmark,
                         satisfied_benchmark,
                         close_benchmark,
                         future_benchmark,
                         past_benchmark,
                         restrict_benchmark,
                         free_benchmark,
                         assign_benchmark,
                         copy_benchmark,
                         shift_benchmark,

        );
        criterion_main!(benches);
    }
}

generate_benchmarks!{
    "udbm", UDBM,
    "rdbm_8bit", RDBM<i8>,
    "rdbm_32bit", RDBM<i32>,
}
