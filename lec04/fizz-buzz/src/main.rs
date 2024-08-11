fn main() {
    println!("Welcome to Fizz Buzz");
    fizz_buzz();
}

fn fizz_buzz() {
    let count = (0..=301).into_iter().fold(0, |mut acc, x| {
        match x {
            x if x % 15 == 0 => {
                print!("fizz buzz\n");
                acc += 1
            }
            x if x % 5 == 0 => print!("buzz\n"),
            x if x % 3 == 0 => print!("fizz\n"),
            _ => (),
        }
        acc
    });

    println!("There were {} fizz buzz", count)
}

//alternative implementation

/* fn fizz_buzz() {
    let mut count: u8 = 0;
    for i in 0..302 {
        match i {
            i if i % 15 => { print!("fizz buzz\n"); count += 1 },
            i if i % 3 == 0 => print!("fizz\n"),
            i if i % 5 == 0 => print!("buzz\n"),
            _ => (),
        }
    }
    print!("fizz buzz occurred {} times\n", count);
} */
