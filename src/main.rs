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
}
