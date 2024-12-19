use arrow::array::{
    ArrayRef, BooleanBuilder, Float64Builder, Int64Builder, StringBuilder,
    TimestampMillisecondBuilder,
};
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use arrow::record_batch::RecordBatch;
use chrono::{DateTime, Duration, Utc};
use parquet::arrow::arrow_writer::ArrowWriter;
use parquet::basic::Compression;
use parquet::file::properties::WriterProperties;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs::File;
use std::sync::Arc;

fn generate_name(rng: &mut impl Rng) -> String {
    const FIRST_NAMES: &[&str] = &[
        "James",
        "Mary",
        "John",
        "Patricia",
        "Robert",
        "Jennifer",
        "Michael",
        "Linda",
        "William",
        "Elizabeth",
        "David",
        "Barbara",
        "Richard",
        "Susan",
        "Joseph",
        "Jessica",
        "Thomas",
        "Sarah",
        "Charles",
        "Karen",
    ];
    const LAST_NAMES: &[&str] = &[
        "Smith",
        "Johnson",
        "Williams",
        "Brown",
        "Jones",
        "Garcia",
        "Miller",
        "Davis",
        "Rodriguez",
        "Martinez",
        "Hernandez",
        "Lopez",
        "Gonzalez",
        "Wilson",
        "Anderson",
        "Thomas",
        "Taylor",
        "Moore",
        "Jackson",
        "Martin",
    ];

    format!(
        "{} {}",
        FIRST_NAMES.choose(rng).unwrap(),
        LAST_NAMES.choose(rng).unwrap()
    )
}

fn generate_company(rng: &mut impl Rng) -> String {
    const PREFIXES: &[&str] = &[
        "Tech",
        "Global",
        "Advanced",
        "Digital",
        "Smart",
        "Future",
        "Innovative",
        "Dynamic",
        "Strategic",
        "Premier",
    ];
    const SUFFIXES: &[&str] = &[
        "Systems",
        "Solutions",
        "Corporation",
        "Industries",
        "Technologies",
        "Enterprises",
        "Group",
        "International",
        "Dynamics",
        "Associates",
    ];

    format!(
        "{} {}",
        PREFIXES.choose(rng).unwrap(),
        SUFFIXES.choose(rng).unwrap()
    )
}

fn main() {
    let sizes = vec![1_000, 100_000, 1_000_000];
    for size in sizes {
        generate_test_file(size).unwrap_or_else(|e| {
            eprintln!("Error generating test file: {}", e);
        });
    }
}

fn generate_test_file(num_rows: usize) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let start_date = Utc::now() - Duration::days(365);

    // Create builders for each column
    let mut id_builder = Int64Builder::new();
    let mut name_builder = StringBuilder::new();
    let mut company_builder = StringBuilder::new();
    let mut salary_builder = Float64Builder::new();
    let mut is_active_builder = BooleanBuilder::new();
    let mut timestamp_builder = TimestampMillisecondBuilder::new();

    // Generate data
    for i in 0..num_rows {
        id_builder.append_value(i as i64);
        name_builder.append_value(generate_name(&mut rng));
        company_builder.append_value(generate_company(&mut rng));
        salary_builder.append_value(rng.gen_range(30000.0..200000.0));
        is_active_builder.append_value(rng.gen_bool(0.9));

        let random_days = rng.gen_range(0..365);
        let timestamp: DateTime<Utc> = start_date + Duration::days(random_days);
        timestamp_builder.append_value(timestamp.timestamp_millis());
    }

    // Create arrays
    let arrays: Vec<ArrayRef> = vec![
        Arc::new(id_builder.finish()),
        Arc::new(name_builder.finish()),
        Arc::new(company_builder.finish()),
        Arc::new(salary_builder.finish()),
        Arc::new(is_active_builder.finish()),
        Arc::new(timestamp_builder.finish()),
    ];

    // Define schema
    let schema = Schema::new(vec![
        Field::new("id", DataType::Int64, false),
        Field::new("name", DataType::Utf8, false),
        Field::new("company", DataType::Utf8, false),
        Field::new("salary", DataType::Float64, false),
        Field::new("is_active", DataType::Boolean, false),
        Field::new(
            "timestamp",
            DataType::Timestamp(TimeUnit::Millisecond, None),
            false,
        ),
    ]);

    // Create RecordBatch
    let batch = RecordBatch::try_new(Arc::new(schema.clone()), arrays)?;

    // Set up Parquet writer properties
    let props = WriterProperties::builder()
        .set_compression(Compression::SNAPPY)
        .build();

    // Create output file
    let file = File::create(format!("test_data_{}_rows.parquet", num_rows))?;

    // Create writer
    let mut writer = ArrowWriter::try_new(file, Arc::new(schema), Some(props))?;

    // Write batch
    writer.write(&batch)?;

    // Close writer
    writer.close()?;

    println!("Generated test file with {} rows", num_rows);
    Ok(())
}
