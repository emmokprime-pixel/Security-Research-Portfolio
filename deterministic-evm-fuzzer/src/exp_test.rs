fn w_exp_approx(x: f64) -> f64 {
    let ln2 = 0.6931471805599453;
    let mut q = ((x + (if x < 0.0 { -ln2 / 2.0 } else { ln2 / 2.0 })) / ln2).floor() as i32;
    let r = x - (q as f64) * ln2;
    
    // 2nd order Taylor: 1 + r + r^2/2
    let exp_r = 1.0 + r + (r * r) / 2.0;
    
    exp_r * (2.0f64.powi(q))
}

fn main() {
    let ln2 = 0.6931471805599453;
    let boundary = ln2 / 2.0;
    
    println!("Checking monotonicity around boundary: {}", boundary);
    
    let step = 1e-10;
    let x1 = boundary - step;
    let x2 = boundary + step;
    
    let y1 = w_exp_approx(x1);
    let y2 = w_exp_approx(x2);
    
    println!("x1: {}, y1: {:.20}", x1, y1);
    println!("x2: {}, y2: {:.20}", x2, y2);
    
    if y2 < y1 {
        println!("VULNERABILITY FOUND: Non-monotonic exponential approximation!");
        println!("Divergence: {}", y1 - y2);
    } else {
        println!("Monotonic in this small range.");
    }
}
