fn main() {
    let number: i64 = 600851475143;

    let result: i64 = prime_factor(number);

    println!("Prime factor {result}");
}

fn prime_factor(x: i64) -> i64 {
    let mut index = 0;
    let mut divisible = 0;

    while index < x {
        if is_prime(index) {
           if x%index == 0 {
               divisible = index;
               println!("Divisible: {divisible}");
           }
        }
        index += 1;
        println!("Index: {index}");
    }

    return divisible;
}

fn is_prime(x: i64) -> bool {
    if x <= 1 {
        return false;
    }
    
    let mut index = 2;

    while index < x {
        if x%index == 0 {
            return false;
        }
        index += 1;
    }

    return true;
}
