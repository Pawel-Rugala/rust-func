fn main() {
    println!("Hello, world!");
    another_func(5, 'c');
    let x = five();
    println!("The value of x is: {}", x);
    let y = plus_one(x);
    println!("The value of y is: {}", y);
    let z = if_exp(5);
    println!("The value of z is: {}", z);
    let za: i32 = if_exp(plus_one(five()));
    println!("The value of za is: {}", za);
    let zb = if_let(true);
    println!("The value of zb is: {}", zb);


    //while fibonnacci
    let fib = nth_fibonacci_while(10);
    println!("The value of fib is: {}", fib);

    //for in fibonnacci
    let fib = nth_fibonacci_forin(10);
    println!("The value of fib is: {}", fib);
}

fn another_func(x: i32, unit: char) {
    println!("Another function. parameter: {x}{unit}");
}

fn five() -> i32 {
    5 //no semicolon means return
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

fn if_exp(x: i32) -> i32 {
    if x == 5 {
        5
    } else {
        10
    }
}

fn if_let (trues: bool) -> i32 {
    let number = if trues {
        5
    } else {
        6
    };
    number
}


fn nth_fibonacci_while(n: i32) -> i32 {
    if n == 0 {
       return 0;
    }

    let mut fib = (0,1);
    let mut count = 1;

    while count < n {
        fib = (fib.1, fib.0 + fib.1);
        count += 1;
    }

    fib.1
}

fn nth_fibonacci_forin(n: i32) -> i32 {
    if n == 0 {
       return 0;
    }

    let mut fib = (0,1);

    for _ in 1..n {
        fib = (fib.1, fib.0 + fib.1);
    }

    fib.1
}
