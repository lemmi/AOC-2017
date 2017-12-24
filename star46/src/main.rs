fn main() {
    let a = 1i64;
    let mut b;
    let mut c;
    let mut h = 0i64;

    b = 81;
    c = b;
    if a != 0 {
        b = b * 100 + 100_000;
        c = b + 17000;
    }

    // Count all non-prime numbers in [b,c]
    while b <= c {
        for d in 2..b {
            if b % d == 0 {
                h += 1;
                break;
            }
        }
        b += 17;
    } // jnz 1 -23
    println!("a: {:7} b: {:7} c: {:7} h: {:7}", a, b, c, h);
}
