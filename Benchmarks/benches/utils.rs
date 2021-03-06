
use criterion::{Bencher, black_box, criterion_group, criterion_main, Criterion, BatchSize};

use std::time::{Duration, Instant};

#[inline]
pub fn bench_function_with_noop<O, N, R>(iters: u64, mut noop: N, mut routine: R) -> Duration
where
    N: FnMut() -> O,
    R: FnMut() -> O,
{
    let start = Instant::now();
    for _i in 0..iters {
        black_box(noop());
    }
    let t_noop = start.elapsed();

    let start = Instant::now();
    for _i in 0..iters {
        black_box(routine());
    }
    let t_routine = start.elapsed();

    if let Some(diff) = t_routine.checked_sub(t_noop) {
        diff
    } else {
        std::time::Duration::from_nanos(0)
    }
}


#[inline]
pub fn iter_noop<O, N, R>(b: &mut Bencher, mut noop: N, mut routine: R)
where
    N: FnMut() -> O,
    R: FnMut() -> O,
{
    b.iter_custom(|iters| {
        bench_function_with_noop(
            iters,
            &mut noop,
            &mut routine,
        )
    });
}


#[inline]
pub fn bench_function_with_noop_batched<I, O, G, N, R>(iters: u64, mut generator: G, mut noop: N, mut routine: R) -> Duration
where
    G: FnMut(u64) -> I,
    N: FnMut(I) -> O,
    R: FnMut(I) -> O,
    I: Clone,
{
    let t_noop: std::time::Duration;
    let t_routine: std::time::Duration;

    /*
    {
        // warm up (probably helps for reserving memory)
        let inputs = black_box((0..iters).map(|i| black_box(generator(i))).collect::<Vec<_>>());
        let mut outputs = Vec::with_capacity(iters as usize);
        let start = Instant::now();
        outputs.extend(inputs.into_iter().map(&mut noop));
        start.elapsed();
        black_box(outputs);
    }
    */
    let inputs = black_box((0..iters).map(|i| black_box(generator(i))).collect::<Vec<_>>());
    {
        // noop
        //let inputs = black_box((0..iters).map(|i| black_box(generator(i))).collect::<Vec<_>>());
        //let mut outputs = Vec::with_capacity(iters as usize);
        //for i in 0..iters as usize {
        //    black_box(&inputs[i]);
        //}
        for x in &inputs {
            black_box(noop(x.clone()));
        }
        let start = Instant::now();
        for x in &inputs {
            black_box(noop(x.clone()));
        }
        //outputs.extend(inputs.into_iter().map(&mut noop));
        t_noop = start.elapsed();
    }
    {
        // routine
        //let inputs = black_box((0..iters).map(|i| black_box(generator(i))).collect::<Vec<_>>());
        //let mut outputs = Vec::with_capacity(iters as usize);
        //for i in 0..iters as usize {
        //    black_box(&inputs[i]);
        //}
        for x in &inputs {
            black_box(routine(x.clone()));
        }
        let start = Instant::now();
        for x in &inputs {
            black_box(routine(x.clone()));
        }
        //outputs.extend(inputs.into_iter().map(&mut routine));
        t_routine = start.elapsed();
    }

    // println!("{} {:?} {:?} {:?}", iters, t_noop, t_routine, t_routine.checked_sub(t_noop));

    if let Some(diff) = t_routine.checked_sub(t_noop) {
        diff
    } else {
        std::time::Duration::from_nanos(0)
    }
}


#[inline]
pub fn iter_noop_batched<I, O, G, N, R>(b: &mut Bencher, mut generator: G, mut noop: N, mut routine: R)
where
    G: FnMut(u64) -> I,
    N: FnMut(I) -> O,
    R: FnMut(I) -> O,
    I: Clone,
{
    b.iter_custom(|iters| {
        bench_function_with_noop_batched(
            iters,
            &mut generator,
            &mut noop,
            &mut routine,
        )
    });
}