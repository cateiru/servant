use std::time::Instant;
use termion::color;

const MAX_FIBONACCI_CALCULATE: usize = 40;

pub fn bench() {
    let start = Instant::now();

    let ans = fibonacci(MAX_FIBONACCI_CALCULATE);

    let end = start.elapsed();

    println!(
        "Benchmark score: {}{}.{:03}s{}",
        color::Fg(color::Magenta),
        end.as_secs(),
        end.subsec_nanos() / 1_000_000,
        color::Fg(color::Reset),
    );
    println!(
        "\tFibonacci calculate length to {}, answer {}.",
        MAX_FIBONACCI_CALCULATE, ans
    );
}

fn fibonacci(i: usize) -> usize {
    match i {
        0..=1 => i,
        _ => fibonacci(i - 1) + fibonacci(i - 2),
    }
}
