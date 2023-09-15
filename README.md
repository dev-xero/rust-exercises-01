# Rust Exercises: 01
Exercises at the end of chapter 03 of the 'Rust Programming Language' book

## Solutions
1. Temperature Conversion (to celsius, fahrenheit and kelvin)  
   Simple temperature conversions using their conversion formulae.
   
   ```rust
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
   ```

2. The nth Fibonacci number in the sequence  
   Returns fib(n) ~ not optimized for speed.
   ```rust
      fn nth_fib(n: u32) -> u32 {
          if n == 0 {
              return 0;
          }
      
          if n == 1 {
              return 1;
          }
      
          let mut curr = 1;
          let mut _prev = 1;
      
          for _ in 4..=n {  // start at 4 since we've handled 0, 1, 1 and 2
              let temp = curr;
      
              curr += _prev;
              _prev = temp;
          }
      
          return curr;
      }
   ```
