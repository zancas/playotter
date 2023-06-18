use plotters::{backend, chart, coord, drawing, style};
use rand::SeedableRng;
use rand_distr::{Distribution, Normal};
use rand_xorshift::XorShiftRng;

const OUT_FILE_NAME: &'static str = "normal-dist.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = drawing::IntoDrawingArea::into_drawing_area(backend::BitMapBackend::new(
        OUT_FILE_NAME,
        (1024, 768),
    ));
    root.fill(&style::colors::WHITE)?;

    let sd = 0.13;
    let random_points: Vec<(f64, f64)> = {
        let norm_dist = Normal::new(0.5, sd).unwrap();
        let mut x_rand = XorShiftRng::from_seed(*b"ANEXAMPLESEED__X");
        let mut y_rand = XorShiftRng::from_seed(*b"ANEXAMPLESEED__Y");
        let x_iter = norm_dist.sample_iter(&mut x_rand);
        let y_iter = norm_dist.sample_iter(&mut y_rand);
        x_iter.zip(y_iter).take(10_000).collect()
    };
    let areas = root.split_by_breakpoints([944], [80]);
    let linspace = coord::combinators::IntoLinspace::step(0.0..1.0, 0.01).use_round();
    let segmented_coord = coord::ranged1d::IntoSegmentedCoord::into_segmented(linspace);
    let mut x_hist_ctx = chart::ChartBuilder::on(&areas[0])
        .y_label_area_size(40)
        .build_cartesian_2d(segmented_coord, 0..250)?;
    todo!()
}
