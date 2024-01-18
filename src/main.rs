fn main() {

    // 43 is the limit for this num type
    let number = 43;


    println!("Result: {}", fib(number));
}

fn fib(number: i32) -> i32 {
    let mut previous = 1;
    let mut i = 2;
    let mut counter = 0;
    let mut num = 0;

    if number == 0 {
        return 0;
    } else if number == 1{
        return 1;
    }
    while counter < number { 
        num = i + previous; 
        previous = i;
        i = num;
        counter += 1;
        println!("{}", i);

    }
    return num;
}
