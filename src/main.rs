fn main() {
    let sum: u32 = (0..)
        .map_while(|i| {
            let n = fib(i);
            (n < 4_000_000).then_some(n)
        })
        .filter(|n| n % 2 == 0)
        .sum();
    println!("{sum}");
}

fn fib(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 2,
        n => fib(n - 1) + fib(n - 2),
    }
}
