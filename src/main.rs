use std::io;



fn main() {
    println!("Conversion");
    println!("1- Convert from Fahrenheit to Celsius");
    println!("2- Convert from Celsius to Fahrenheit");
    println!("3- Exit");

    loop {
        println!("Make your choice");

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Invalid choice");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid choice");
                continue;
            }
        };

        if choice == 1 {
            println!("Enter temperature in Fahrenheit:");

            let mut fahrenheit = String::new();

            io::stdin()
                .read_line(&mut fahrenheit)
                .expect("Invalid temperature");

            let fahrenheit: f32 = match fahrenheit.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temperature");
                    continue;
                }
            };

            let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
            println!("{} Fahrenheit is equal to {} Celsius", fahrenheit, celsius);
        } else if choice == 2 {
            println!("Enter temperature in Celsius:");

            let mut celsius = String::new();

            io::stdin()
                .read_line(&mut celsius)
                .expect("Invalid temperature");

            let celsius: f32 = match celsius.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Invalid temperature");
                    continue;
                }
            };

            let fahrenheit = celsius * 9.0 / 5.0 + 32.0;
            println!("{} Celsius is equal to {} Fahrenheit", celsius, fahrenheit);
        } else if choice == 3 {
            println!("Exiting...");
            break;
        } else {
            println!("Invalid choice");
        }
    }
}
