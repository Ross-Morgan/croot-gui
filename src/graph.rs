use num_complex::Complex64;
use plotters::prelude::*;

pub fn generate_graph<T: AsRef<std::path::Path> + ?Sized>(roots: Vec<Complex64>, filename: &T, dimensions: (u32, u32)) -> Result<(), Box<dyn std::error::Error>> {
    let draw_area = BitMapBackend::new(filename, dimensions).into_drawing_area();

    let radius = roots.first().map(|c| c.norm()).expect("No roots provided");
    let axes_range = (radius * -1.2)..(radius * 1.2);

    draw_area.fill(&WHITE)?;

    // Create chart
    let mut chart = ChartBuilder::on(&draw_area)
        .caption(format!("{} roots of {}", roots.len(), roots.first().copied().unwrap_or_default().powi(roots.len() as i32)), ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(axes_range.clone(), axes_range.clone())?;

    // Add grid to background
    chart.configure_mesh().draw()?;

    // Draw line on y-axis
    chart.draw_series(LineSeries::new(
        (((radius * -1.5) as i32)..=((radius * 1.5) as i32))
            .map(|x| x as f64)
            .map(|y| (0.0, y)),
        BLACK,
    ))?;

    // Draw line on x-axis
    chart.draw_series(LineSeries::new(
        (((radius * -1.5) as i32)..=((radius * 1.5) as i32))
            .map(|x| x as f64)
            .map(|x| (x, 0.0)),
        BLACK,
    ))?;

    // Draw line from origin to each root
    for root in roots.iter() {
        chart
            .draw_series(LineSeries::new(
                (0..=100)
                    .map(|x| x as f64 / 100.0)
                    .map(|x| (x * root.re, x * root.im)),
                BLACK.stroke_width(2),
            ))?;
    }

    // Draw circles on each root
    chart.draw_series(roots.iter().map(|c| Circle::new((c.re, c.im), 5, BLACK.filled())))?;

    // Draw circle at origin
    chart.plotting_area().draw(&Circle::new((0.0, 0.0), 5, BLACK.stroke_width(5).filled()))?;

    // Generate graph
    draw_area.present()?;

    Ok(())
}

#[cfg(feature = "gui")]
pub fn show_graph(roots: Vec<Complex64>, dimensions: (u32, u32)) -> Result<(), Box<dyn std::error::Error>> {
    use std::path::PathBuf;

    use crate::app::show_image;

    let path: PathBuf = [".", "temp.png"].iter().collect();

    generate_graph(roots, "temp.png", dimensions)?;
    show_image("Complex Root Graph", path, dimensions)?;
    std::fs::remove_file("temp.png")?;

    Ok(())
}