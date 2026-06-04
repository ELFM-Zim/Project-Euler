fn main() {

    let mut index = 0;
    let mut sum: i64 = 0;

    while index < 365000 {
        let temp: i64 = square(index);
        if is_odd(temp)
        {
            sum += temp;
        }
        index += 1;
    }
    println!("Total value: {sum}");
} 

fn square(n: i64) -> i64 {
   return n*n;
}

fn is_odd(n: i64) -> bool{
    if n%2 == 0 {
        return false;
    }

    return true;
}
