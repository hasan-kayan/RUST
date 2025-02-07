use tergraph::{draw_bar_chart, draw_line_chart, clear_screen};

fn main() {
    let data = vec![3, 7, 10, 5, 2, 8, 6];

    clear_screen();
    println!("Bar Chart:");
    draw_bar_chart(&data, 10);

    println!("\nLine Chart:");
    draw_line_chart(&data);
}
