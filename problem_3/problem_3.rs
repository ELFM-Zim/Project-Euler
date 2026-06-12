fn main() {
    let number: i64 = 600851475143;

    let result: i64 = prime_factor(number);

    println!("Prime factor {result}");
}

fn prime_factor(mut a: i64) -> i64 {
    let mut b: i64 = 2;

    while a > b {
        if a%b == 0 {
            a = a/b;
            b = 2;
        }
        else {
            b+=1;
        }
    }

    return b;
}


