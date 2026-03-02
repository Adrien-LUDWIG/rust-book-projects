fn fibonacci(n: u32) -> u64 {
    let mut n0 = 0;
    let mut n1 = 1;

    for _ in 0..n {
        let n2 = n0 + n1;
        n0 = n1;
        n1 = n2;
    }

    n0
}

fn main() {
    println!("{}", fibonacci(81));
}
