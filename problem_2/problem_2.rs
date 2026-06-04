fn main() {
    
    let x: i64 = fibonacci();
    is_even(x);
    println!("Fibonacci: {x}");

}

fn fibonacci() -> i64 {
    let mut sum: i64 = 0;
    let mut fib: i64 = 1;
    let mut temp: i64 = 1;

    fib += temp;
    sum += fib;
    loop{
        let aux: i64 = temp;
        temp = fib;

        if fib+aux > 4000000
        {
            break;
        }

        fib += aux;

        if is_even(fib) {
            sum += fib;
        }
    }

    return sum;
    
}

fn is_even(x: i64) -> bool {
    if x%2 == 0 {
        return true;
    }

    return false;
}
