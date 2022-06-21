use std::io;

fn main() {
    println!("Please enter your weight (kg): ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let weight:f32 = input.trim().parse().unwrap();


    let mars_weight:f32 = calculate_weight_on_mars(weight);
    println!("The weight on mars is {} ", mars_weight);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    weight/9.81 * 3.711
}
