use crate::datapoint::Datapoint;
use crate::plotter::plotcolors::PlotColors;
use crate::plotter::util::*;
use chrono::prelude::*;
use plotters::prelude::*;

pub fn categorical_plot(dataset: Vec<(Vec<Datapoint>, Vec<Vec<String>>)>) -> Option<String> {
    let plot_x_start = 0;
    let plot_x_end = dataset.len() + 1;

    let titled_datasets = into_categorical(dataset);
    if titled_datasets.len() == 0 {
        return None;
    }
    let (lower, upper) = apply_margin(get_total_upper_lower(titled_datasets.clone()));

    let filename = generate_filename(Local::now());
    let location: String = format!("generated/{}", filename);
    let plot_colors = PlotColors::new();
    let root = BitMapBackend::new(&location, (640, 480)).into_drawing_area();
    root.fill(plot_colors.background()).unwrap();

    let (title, font_size) = generate_title(titled_datasets.clone());

    let mut chart = ChartBuilder::on(&root)
        .caption(
            title,
            ("sans-serif", font_size)
                .with_color(plot_colors.textcolor())
                .into_text_style(&root),
        )
        .margin(10)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(plot_x_start..plot_x_end, lower..upper)
        .unwrap();

    chart
        .configure_mesh()
        .label_style(plot_colors.highlight())
        .axis_style(plot_colors.textcolor())
        .bold_line_style(plot_colors.highlight())
        .light_line_style(plot_colors.darklight())
        .x_label_formatter(&|i| {
            if *i == 0 {
                return "".to_string();
            }
            match titled_datasets.get(i - 1) {
                Some((_, title)) => title.to_owned(),
                None => "".to_string(),
            }
        })
        .draw()
        .unwrap();

    for (i, titled_data) in titled_datasets.into_iter().enumerate() {
        chart
            .draw_series(
                titled_data
                    .0
                    .iter()
                    .map(|value| Circle::new((i + 1, *value), 5, plot_colors.labelstyle().clone())),
            )
            .unwrap();
    }

    return Some(filename);
}

fn generate_title(titled_data: Vec<(Vec<f64>, String)>) -> (String, u32) {
    let mut title: String = titled_data[0].1.clone();
    let mut counter = 1;
    while counter < titled_data.len() {
        title = format!("{} vs {}", title, titled_data[counter].1);
        counter += 1;
    }
    let font_size = 10 + (40 / counter as u32);
    return (title, font_size);
}

fn into_categorical(datasets: Vec<(Vec<Datapoint>, Vec<Vec<String>>)>) -> Vec<(Vec<f64>, String)> {
    let mut collector: Vec<(Vec<f64>, String)> = Vec::new();
    for (datapoints, query) in datasets {
        let values: Vec<f64>;
        match get_numeric_data(&datapoints) {
            Some((_, data)) => values = data,
            None => continue,
        };
        let title = collect_query(query);
        collector.push((values, title));
    }
    return collector;
}

fn get_total_upper_lower(titled_data: Vec<(Vec<f64>, String)>) -> (f64, f64) {
    let mut collector: Vec<f64> = Vec::new();
    for (values, _) in titled_data {
        let (lower, upper) = get_upper_lower(&values);
        collector.push(lower);
        collector.push(upper);
    }
    get_upper_lower(&collector)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::datastore::Datastore;

    #[test]
    fn plotting_queries_with_no_results_returns_none() {
        let mut collector: Vec<(Vec<Datapoint>, Vec<Vec<String>>)> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +tea");
        datastore.add_datapoint("8 hours +sleep +tea");
        collector.push(datastore.query("totally not findable"));

        let result = categorical_plot(collector);

        assert_eq!(result, None);
    }

    #[test]
    fn generate_title_generates_expected_title() {
        let mut collector: Vec<(Vec<Datapoint>, Vec<Vec<String>>)> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +tea");
        datastore.add_datapoint("8 hours +sleep +tea");
        datastore.add_datapoint("6 hours +sleep +cola");
        datastore.add_datapoint("5.5 hours +sleep +cola");
        collector.push(datastore.query("sleep coffee"));
        collector.push(datastore.query("sleep tea"));
        collector.push(datastore.query("sleep cola"));

        let titled_data = into_categorical(collector);
        let (title, _) = generate_title(titled_data);

        assert_eq!(title, "sleep, coffee vs sleep, tea vs sleep, cola")
    }

    #[test]
    fn generate_title_generates_appropriate_text_size_for_display() {
        let mut collector: Vec<(Vec<Datapoint>, Vec<Vec<String>>)> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +coffee");
        datastore.add_datapoint("7 hours +sleep +tea");
        datastore.add_datapoint("8 hours +sleep +tea");
        datastore.add_datapoint("6 hours +sleep +cola");
        datastore.add_datapoint("5.5 hours +sleep +cola");
        collector.push(datastore.query("sleep coffee"));
        collector.push(datastore.query("sleep tea"));
        collector.push(datastore.query("sleep cola"));

        let titled_data = into_categorical(collector);
        let (_, size) = generate_title(titled_data);

        assert_eq!(size, 23)
    }

    #[test]
    fn get_total_upper_lower_gets_upper_and_lower_of_vector_of_titled_data() {
        let mut collector: Vec<(Vec<Datapoint>, Vec<Vec<String>>)> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        collector.push(datastore.query("coffee"));
        collector.push(datastore.query("tea"));

        let titled_data = into_categorical(collector);
        let (lower, upper) = get_total_upper_lower(titled_data);

        assert_eq!(lower, 6.0);
        assert_eq!(upper, 8.0);
    }

    #[test]
    fn into_categorical_takes_a_vector_of_retrieved_datasets_and_queries_and_returns_titled_data_vectors(
    ) {
        let mut collector: Vec<(Vec<Datapoint>, Vec<Vec<String>>)> = Vec::new();
        let datastore = Datastore::new();
        datastore.add_datapoint("6 hours +coffee");
        datastore.add_datapoint("7 hours +coffee");
        datastore.add_datapoint("7 hours +tea");
        datastore.add_datapoint("8 hours +tea");
        collector.push(datastore.query("coffee"));
        collector.push(datastore.query("tea"));

        let titled_data = into_categorical(collector);

        assert_eq!(titled_data[0].0[0], 6.0);
        assert_eq!(titled_data[0].0[1], 7.0);
        assert_eq!(titled_data[1].0[0], 7.0);
        assert_eq!(titled_data[1].0[1], 8.0);
        assert_eq!(titled_data[0].1, "coffee".to_string());
        assert_eq!(titled_data[1].1, "tea".to_string());
    }
}
