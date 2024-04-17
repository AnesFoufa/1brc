use polars::prelude::*;
use polars_lazy::frame::LazyCsvReader;
use std::sync::Arc;
pub fn solve(path: &str) {
    let schema = Schema::from_iter(
        vec![
            Field::new("city", DataType::String),
            Field::new("temperature", DataType::Float64),
        ]
        .into_iter(),
    );
    let reader = LazyCsvReader::new(path)
        .has_header(false)
        .with_separator(b';')
        .with_schema(Some(Arc::new(schema)));
    let frame = reader.finish().unwrap();
    let stations = frame
        .group_by([col("city")])
        .agg([
            col("temperature").mean().alias("mean_tempeature"),
            col("temperature").max().alias("max_temperature"),
            col("temperature").min().alias("min_temperature"),
        ])
        .sort(["city"], Default::default())
        .with_streaming(true)
        .collect()
        .unwrap();

    let columns = stations.get_columns();
    let cities = columns[0].str().unwrap().iter().map(|x| x.unwrap());
    let mean_temperatures = columns[1].f64().unwrap().iter().map(|x| x.unwrap());
    let max_temperatures = columns[2].f64().unwrap().iter().map(|x| x.unwrap());
    let min_temperatures = columns[3].f64().unwrap().iter().map(|x| x.unwrap());
    for (((city, min_temperature), mean_temperatures), max_temperature) in cities
        .zip(min_temperatures)
        .zip(mean_temperatures)
        .zip(max_temperatures)
    {
        println!(
            "{}={:.1}/{:.1}/{:.1}",
            city, min_temperature, mean_temperatures, max_temperature
        );
    }
}
