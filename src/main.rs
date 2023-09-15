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

fn nth_fib(n: u32) -> u32 {
    if n == 0 {
        return 0;
    }

    if n == 1 {
        return 1;
    }

    let mut curr = 1;
    let mut _prev = 1;

    for _ in 4..=n {
        let temp = curr;

        curr += _prev;
        _prev = temp;
    }

    return curr;
}

fn main() {
    println!("Rust exercises");

    println!("\nTemperature Conversion");
    println!("---");

    let body_temp = convert_temperature(97, Temp::Celsius);
    let absolute_zero = convert_temperature(-273, Temp::Kelvin);
    let water_boiling_point = convert_temperature(100, Temp::Fahrenheit);

    println!("96F to celsius is: {body_temp}C");
    println!("Absolute zero, {absolute_zero}K is: -273 C");
    println!("The boiling point of pure water is: {water_boiling_point} F");

    println!("\nFibonacci Sequences");
    println!("---");

    let fifth_fib = nth_fib(5);
    let tenth_fib = nth_fib(10);

    println!("The fifth fib term is: {fifth_fib}");
    println!("The tenth fib term is: {tenth_fib}");
}
