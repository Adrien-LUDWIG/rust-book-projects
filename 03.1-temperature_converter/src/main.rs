fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 1.8 + 32.
}

fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.) / 1.8
}

fn main() {
    let celsius = 42.1;
    let fahrenheit = celsius_to_fahrenheit(celsius);
    let celsius_bis = fahrenheit_to_celsius(fahrenheit);
    println!("{celsius} -> {fahrenheit} -> {celsius_bis}");
}
