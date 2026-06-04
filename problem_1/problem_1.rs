fn main(){
    let mut index = 0;
    let mut sum = 0;

    while index < 1000 {
        if is_mult_of_3_or_5(index) {
            sum += index;
        }
        index += 1;   
    }

    println!("Soma total: {sum}");
}

fn is_mult_of_3_or_5(n: i32) -> bool{
    if n%3 == 0 || n%5 == 0 {
        return true;
    }

    return false;
}
