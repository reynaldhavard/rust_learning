use std::io;

fn main() {
    println!("Choose the program:");
    println!("1) Convert a temperature from Fahrenheit to Celsius.");
    println!("2) Generate the nth Fibonacci number.");
    println!("3) Print the lyrics of \"The Twelve Days of Christmas\".");

    println!("Choose a value between 1 and 3.");

    let mut choice_number = 0;
    while choice_number < 1 || choice_number > 3 {
        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        choice_number = choice.trim().parse().expect("Incorrect value");

        if choice_number < 1 || choice_number > 3 {
            println!("Choose a value between 1 and 3.");
        }
    }

    if choice_number == 1 {
        convert_temp_f_to_c();
    } else if choice_number == 2 {
        get_nth_fibonacci_number();
    } else if choice_number == 3 {
        lyrics_twelve_days_of_christmas();
    }
}

fn convert_temp_f_to_c() {
    println!("Convert temperature from Fahrenheit to Celsius.");
    let mut f_temp = String::new();
    io::stdin()
        .read_line(&mut f_temp)
        .expect("Failed to read line.");
    let f_temp: f32 = f_temp.trim().parse().expect("Failed to parse.");
    println!("Your temperature in Fahrenheit is: {}°F.", f_temp);

    let c_temp = (f_temp - 32.0) / 1.8;
    println!("The temperature in Celsius is: {}°C.", c_temp);
}

fn get_nth_fibonacci_number() {
    println!("Get the nth Fibonacci number.");
    println!("Choose a value of n:");
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line.");
    let n: u32 = n.trim().parse().expect("Failed to parse.");

    let res;
    if n == 0 {
        res = 0;
    } else if n == 1 {
        res = 1;
    } else {
        let mut a = 0;
        let mut b = 1;
        let mut index = 1;
        while index < n {
            let tmp = a + b;
            a = b;
            b = tmp;
            index += 1;
        }
        res = b;
    }

    println!("The {n}-th Fibonacci number is {res}.");

}

fn lyrics_twelve_days_of_christmas() {
    println!("The lyrics of \"The Twelve Days of Christmas\".\n");
    let n_verses = 12;
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", 
        "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let lines = [
        "Twelve drummers drumming,",
        "Eleven pipers piping,",
        "Ten lords a-leaping,",
        "Nine ladies dancing,",
        "Eight maids a-milking,",
        "Seven swans a-swimming,",
        "Six geese a-laying,",
        "Five golden rings,",
        "Four calling birds,",
        "Three French hens,",
        "Two turtle doves,"
    ];

    let mut index = 0;
    while index < n_verses {
        println!("On the {} day of Christmas,", days[index]);
        println!("My true love gave to me");
        if index == 0 {
            println!("A partridge in a pear tree.\n")
        } else {
            let mut line_index = lines.len() - index;
            while line_index < lines.len() {
                println!("{}", lines[line_index]);
                line_index += 1;
            }
            println!("And a partridge in a pear tree.\n")
        }
        index += 1;
    }
}