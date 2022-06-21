fn main() {
    let mut mars_weight:f32 = calculate_weight_on_mars(100.0);
    mars_weight = mars_weight * 1000.0;
    println!("The weight on mars is {} grams", mars_weight);
}

fn calculate_weight_on_mars(weight:f32) -> f32 {
    weight/9.81 * 3.711
}