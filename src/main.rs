fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // second loop example

    let mut count_two = 0;
    'counting_up: loop {
        println!("count_two = {count_two}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count_two == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count_two += 1;
    }
    println!("End count_two = {count_two}");

    let mut number_three = 3;

    while number_three != 0 {
        println!("{number_three}");
        number_three -= 1;
    }

    println!("LIFTOFF!!!!");

    let a = [10, 20, 30, 40, 50];
    let mut index_four = 0;

    while index_four < 5 {
        println!("the value is: {}", a[index_four]);
        index_four += 1;
    }

    for element_four in a {
        println!("the value is: {element_four}");
    }

    for number_five in (1..4).rev() {
        println!("{number_five}!");
    }
    println!("LIFT OFF!!!!!");
}
