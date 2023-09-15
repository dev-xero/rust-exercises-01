enum Temp {
    Celsius,
    Fahrenheit,
    Kelvin
}

fn convert_temperature(temp: i32, to: Temp) -> i32 {
    fn to_celsius(temp: i32) -> i32 {
        return (temp - 32) * 5/9;
    }

    fn to_fahrenheit(temp: i32) -> i32 {
        return (temp * 9/5) + 32;
    }

    fn to_kelvin(temp: i32) -> i32 {
        return temp + 273;
    }

    return match to {
        Temp::Celsius => { to_celsius(temp) },
        Temp::Fahrenheit => { to_fahrenheit(temp) }
        Temp::Kelvin => { to_kelvin(temp) }
    };
}

fn main() {
    println!("Rust exercises");

    println!("\nTemperature Conversion");
    println!("---");

    let body_temp = convert_temperature(97, Temp::Celsius);
    println!("96F to celsius is: {body_temp}C");
}
