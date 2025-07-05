// NAME:HIMASISH GHOSAL; ROLL NO:MA24M010
// This code implements the Euler method for solving a given IVP.
// The function `f` defines the ODE, and the `euler` function performs the numerical integration.

fn f(t: f64, y: f64) -> f64 {
    t.cos() - y
}

fn euler(a: f64, b: f64, n: usize, y0: f64) -> (Vec<f64>, Vec<f64>) {
    let h = (b - a) / n as f64;
    let mut t_vals = vec![a];
    let mut y_vals = vec![y0];

    for i in 0..n {
        let t = t_vals[i];
        let y = y_vals[i];
        let t_next = t + h;
        let y_next = y + h * f(t, y);
        t_vals.push(t_next);
        y_vals.push(y_next);
    }

    (t_vals, y_vals)
}


// The main function initializes the parameters and calls the euler function to compute the solution.
fn main() {
    println!("Euler Method Solution with n = 20 is : ");
    println!("  t   ,   y");
    let (t, y) = euler(0.0, 5.0, 20, 1.0);

    for (ti, yi) in t.iter().zip(y.iter()) {
        println!("{:.4}, {:.4}", ti, yi);
    }
}
