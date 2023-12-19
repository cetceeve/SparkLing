use polars::prelude::*;

fn main() {
    let mut schema = Schema::new();
    schema.with_column("route_short_name".into(), DataType::Utf8);
    let schema = Arc::new(schema);
    let df = CsvReader::from_path("./sample_data/2023-12-03T22_45_32-rt-data.csv.gz")
        .unwrap()
        .has_header(true)
        .with_dtypes(Some(schema))
        .finish()
        .unwrap();
    let df = df.lazy()
        .with_column((col("timestamp") * lit(1000)).cast(DataType::Datetime(TimeUnit::Milliseconds, None)))
        .collect()
        .unwrap();
    println!("{}", df);
}
