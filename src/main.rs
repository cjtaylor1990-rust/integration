fn main() {
    let x_min = 0.0;
    let x_max = 1.0;
    let delta_x = 0.001;
    println!("Answer: {}", euler_integration(test_fn, x_min, x_max, delta_x));
}

fn test_fn(x: f64) -> f64 {
    x * x
}

fn euler_integration(fn_to_integrate: fn(f64) -> f64, x_min: f64, x_max: f64, delta_x: f64) -> f64 {
    /* Basic Euler discrete integration function.
     *
     * Given a 1-D function [f(x) = y], a min, a max, and an integration delta, find the
     * integral of the function between the limits.
     */
    let mut result = 0.0;
    let mut x = x_min;
    while x < x_max {
        result += fn_to_integrate(x)*delta_x;
        x += delta_x;
    }
    result
}
