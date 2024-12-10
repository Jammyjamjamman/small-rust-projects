use std::f64::consts::PI;


fn factorial(n: u64) -> f64 {
    if n <= 1 {
        1.
    }
    else {
        n as f64*factorial(n-1)
    }
}


fn sin(angle: f64) -> f64 {
    (0..16).into_iter()
        .rev()
        .fold(0., |acc, n| acc + ((-1i8).pow(n) as f64 * angle.powi(2*n as i32+1))/(factorial(2*n as u64+1) as f64))
}

fn tan(angle: f64) -> f64 {
    angle/(1. + (1..12)
    .into_iter()
    .rev()
    .fold(0., |acc, n| -(angle*angle)/((n as f64*2.+1.)+acc)))
}

fn deg2rad(degrees: f64) -> f64 {
    degrees * PI / 180.
}
fn main() {
    println!("sin: {}", sin(4.));
    println!("tan: {}", tan(deg2rad(89.999999999)));
    println!("sin: {}", sin(deg2rad(180.)));
    println!("sin: {}", sin(deg2rad(0.)));
    println!("factorial: {}", factorial(5));
}
