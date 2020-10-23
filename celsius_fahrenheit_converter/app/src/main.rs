fn main() {
    let is_celsius = true;
    let temperature = 20.0;

    let converted_temperature = if is_celsius {
        convert_celsius_to_fahrenheit(temperature)
    } else {
        convert_fahrenheit_to_celsius(temperature)
    };

    println!("Your temperature of {} is converted to: {}", temperature, converted_temperature);
}

fn convert_fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) / 9.0 * 5.0
}

fn convert_celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius / 5.0 * 9.0 + 32.0
}
