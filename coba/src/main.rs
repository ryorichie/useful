use std::io::{stdout, Write};
use std::thread;
use std::time::Duration;

const WIDTH: usize = 40;
const HEIGHT: usize = 20;

fn main() {
    let mut angle: f64 = 0.0;

    loop {
        // Clear the console
        print!("{}[2J", 27 as char);

        // Print the donut
        for j in 0..HEIGHT {
            for i in 0..WIDTH {
                let x = (i as f64 * angle.cos()) as i32;
                let y = (j as f64 * angle.sin()) as i32;
                let distance = (x as f64 * x as f64 + y as f64 * y as f64).sqrt();

                let symbol = if distance < WIDTH as f64 / 4.0 {
                    '*'
                } else if distance < WIDTH as f64 / 2.0 {
                    '#'
                } else if distance < WIDTH as f64 * 3.0 / 4.0 {
                    '.'
                } else {
                    ' '
                };

                print!("{}", symbol);
            }
            println!();
        }

        angle += 0.1;

        // Sleep for 3 seconds
        thread::sleep(Duration::from_millis(50));
    }
}
