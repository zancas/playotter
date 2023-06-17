use plotters::style::colors::WHITE;
use plotters::{backend, drawing};

const OUT_FILE_NAME: &'static str = "normal-dist.png";
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = drawing::IntoDrawingArea::into_drawing_area(backend::BitMapBackend::new(
        OUT_FILE_NAME,
        (1024, 768),
    ));
    root.fill(&WHITE)?;

    todo!()
}
