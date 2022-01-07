use std::io;

fn celsius_2_fahrenheit (temp_ : f64) -> f64{
    (temp_ * (9 as f64)  / (5 as f64 )) +   32 as f64
}

fn fahrenheit_2_celsius (temp_ : f64) -> f64{
    (temp_ - 32 as f64) * (5 as f64 / 9 as f64)
}

fn main() {
    println!("Welcome to temperature converter!\\n");
    'reading_option: loop {
        println!("Choose your option:");
        println!("\t 1 - Convert Fahrenheit to Celsius");
        println!("\t 2 - Convert Celsius to Fahrenheit");
        println!("\t q - Quit");

        let mut option = String::new();

        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");

        if "1" == option.trim() {            
            println!("Input temperature:");
            let mut temp_str = String::new();
            io::stdin()
                .read_line(&mut temp_str)
                .expect("Failed to read line");

                let temp: f64 = match temp_str.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };
                
            println!("The temperature in Celsius is {}C", fahrenheit_2_celsius(temp));
            
        } 
        else if "2" == option.trim() {
            println!("Input temperature:");
            let mut temp_str = String::new();
            io::stdin()
                .read_line(&mut temp_str)
                .expect("Failed to read line");
            
            let temp: f64 = match temp_str.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
            
            println!("The temperature in Fahrenheit is {}F", celsius_2_fahrenheit(temp));

        }
        else if "q" == option.trim() {
            println!("Bye!");

            break 'reading_option;
        }
        else
        {
            println!("Invalid option");
        }

    }
}
