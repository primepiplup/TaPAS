use crate::datapoint::Datapoint;
use plotters::prelude::*;

pub fn basic_plot(data: &Vec<Datapoint>) -> Result<String, Box<dyn std::error::Error>> {
    let filename = "image.png";
    let location: String = format!("generated/{}", filename);

    let (lower_date, upper_date): (i64, i64) = get_daterange(data);

    let root = BitMapBackend::new(&location, (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption("First chart", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(lower_date..upper_date, 0..100)?;

    chart.configure_mesh().draw()?;

    root.present()?;

    Ok(filename.to_owned())
}

fn get_daterange(data: &Vec<Datapoint>) -> (i64, i64) {
    let dates: Vec<i64> = data
        .into_iter()
        .map(|point| point.get_datetime().timestamp())
        .collect();
    if dates.len() > 1 {
        let lower = dates[0];
        let upper = dates[dates.len() - 1];
        (lower, upper)
    } else if dates.len() == 1 {
        (dates[0], dates[0])
    } else {
        (0, 0)
    }
}

fn get_upper_lower(points: Vec<i64>) -> (i64, i64) {
    (0, 0)
}
