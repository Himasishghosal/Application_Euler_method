// NAME:HIMASISH GHOSAL ; ROLL NO:MA24M010
// This code implements the Euler method with n=100 for solving the IVP .
// It compares the numerical solution with the exact analytical solution and writes the results to a CSV file.

use std::fs::File;
use std::io::{Write, BufWriter};

// Define the ODE: y' = cos(t)-y
fn f(t: f64, y: f64) -> f64 {
    t.cos()- y
}

// Euler's Method
fn euler_method(n: usize, t0: f64, t_end: f64, y0: f64) -> Vec<(f64, f64)> {
    let h = (t_end - t0) / n as f64;
    let mut result = Vec::new();
    let mut t = t0;
    let mut y = y0;

    result.push((t, y));

    for _ in 0..n {
        y += h * f(t, y);
        t += h;
        result.push((t, y));
    }

    result
}

// Exact Analytical solution: (sin(t) + cos(t) + exp(-t)) / 2 as Computed in the python code
fn exact_solution(t: f64) -> f64 {
    (t.sin() + t.cos() + (-t).exp()) / 2.0
}

fn main() {
    let n = 100;
    let t0 = 0.0;
    let t_end = 5.0;
    let y0 = 1.0;

    let numeric = euler_method(n, t0, t_end, y0);

    // To Write the results to a CSV file
    let file = File::create("euler_comparison.csv").expect("Could not create file");
    let mut writer = BufWriter::new(file);

    writeln!(writer, "t,numerical,exact,error").unwrap();

    for (t, y_num) in numeric {
        let y_exact = exact_solution(t);
        let error = (y_exact - y_num).abs();
        writeln!(writer, "{:.5},{:.5},{:.5},{:.5}", t, y_num, y_exact, error).unwrap();
    }

    println!("Comparison data written to 'euler_comparison.csv'");
}
