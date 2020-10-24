use text_io::read;

fn main() {
    let is_celsius = is_input_celsius();
    let temperature = get_input_temperature();

    if is_celsius {
        let fahrenheit = convert_celsius_to_fahrenheit(temperature);
        println!("{} Celsius are: {:0.1} Fahrenheit", temperature, fahrenheit);
    } else {
        let celsius = convert_fahrenheit_to_celsius(temperature);
        println!("{} Fahrenheit are: {:0.1} Celsius", temperature, celsius);
    };
}

fn is_input_celsius() -> bool {
    println!("What temperature unit do you want to convert?\nType \"c\" for Celsius or \"f\" for Fahrenheit.");
    let unit: String = read!();

    if unit == "c" {
        println!("Chosen unit: Celsius");
        return true;
    }

    println!("Chosen unit: Fahrenheit");
    false
}

fn get_input_temperature() -> f64 {
    println!("What temperature do you want to convert?");
    // todo: handle non float inputs
    read!()
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 9.0 * 5.0
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius / 5.0 * 9.0 + 32.0
}
