// use plotters::prelude::*;

// const OUT_FILE_NAME: &str = "image.png";

// const WIDTH: u32 = 1600;
// const HEIGHT: u32 = 1600;

// fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let radius = 3_f64;
//     // let roots = root(81.0, 4);

//     let root = BitMapBackend::new(OUT_FILE_NAME, (WIDTH, HEIGHT)).into_drawing_area();

//     root.fill(&WHITE)?;


//     let mut chart = ChartBuilder::on(&root)
//         .caption("x²+y²=r²", ("sans-serif", 50).into_font())
//         .build_cartesian_2d(-5f64..5f64, -5f64..5f64)?;

//     chart.configure_mesh().draw()?;

//     chart
//         .draw_series(LineSeries::new(
//             (-(radius as i32 * 100)..=(radius as i32 * 100))
//                     .map(|y| y as f64 / 100.0)
//                     .map(|y| (y, (radius.powi(2) - y.powi(2)).sqrt())),
//             &RED
//         ))?;

//     chart
//         .draw_series(LineSeries::new(
//             (-(radius as i32 * 100)..=(radius as i32 * 100))
//                     .map(|y| y as f64 / 100.0)
//                     .map(|y| (y, -(radius.powi(2) - y.powi(2)).sqrt())),
//             &RED
//         ))?;

//     root.present()?;

//     Ok(())
// }


use croot_gui::graph::generate_graph;

fn main() {
    generate_graph("graph.png", (1000, 1000)).unwrap();
}
