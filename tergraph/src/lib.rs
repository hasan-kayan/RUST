use crossterm::{execute, style::{Color, PrintStyledContent, Stylize}};
use std::io::{self, Write};

/// Draws a simple horizontal bar chart in the terminal.
/// - `values`: The dataset to plot.
/// - `max_width`: The width of the graph.
pub fn draw_bar_chart(values: &[u32], max_width: usize) {
    let max_value = *values.iter().max().unwrap_or(&1);

    for &value in values {
        let bar_width = (value as f64 / max_value as f64 * max_width as f64).round() as usize;
        let bar = "█".repeat(bar_width);

        // Print the value on the left, align it to 4 spaces
        print!("{:>3} | ", value);
        
        execute!(
            io::stdout(),
            PrintStyledContent(bar.with(Color::Green))
        ).unwrap();

        println!(); // Move to the next line
    }

    // Print separator line under bars
    println!("{}", "-".repeat(max_width + 6));
}

pub fn draw_line_chart(values: &[u32]) {
    let max_value = *values.iter().max().unwrap_or(&1);
    let height = 10; // Fixed graph height in terminal

    for level in (0..=height).rev() {
        let y_label = (max_value as f64 * (level as f64 / height as f64)).round() as usize;
        print!("{:>3} | ", y_label); // Left-side Y-axis labels

        for &value in values {
            let normalized = (value as f64 / max_value as f64 * height as f64).round() as usize;

            if level <= normalized {
                print!("● "); // Plot the point and fill below
            } else {
                print!("  "); // Empty space
            }
        }
        println!();
    }

    // Draw X-axis
    print!("    "); // Space for Y-axis labels
    println!("{}", "-".repeat(values.len() * 2));

    // Draw X-axis labels (indices)
    print!("    "); // Align with Y-axis labels
    for i in 0..values.len() {
        print!("{:>2} ", i);
    }
    println!();
}

/// Clears the terminal screen before drawing new graphics.
pub fn clear_screen() {
    print!("\x1B[2J\x1B[H");
}
