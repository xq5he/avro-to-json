use avro_to_json::convert_avro_to_json_with_color;
use anyhow::{Context, Result};
use clap::{Arg, Command};

fn main() -> Result<()> {
    let matches = Command::new("avro-to-json")
        .version("0.1.0")
        .author("Your Name")
        .about("Converts Avro files to JSON format")
        .arg(
            Arg::new("input")
                .short('i')
                .long("input")
                .value_name("FILE")
                .help("Input Avro file")
                .required(true),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("FILE")
                .help("Output JSON file (optional, defaults to stdout)"),
        )
        .arg(
            Arg::new("pretty")
                .short('p')
                .long("pretty")
                .help("Pretty print JSON output")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("array")
                .short('a')
                .long("array")
                .help("Output as JSON array instead of newline-delimited JSON")
                .action(clap::ArgAction::SetTrue),
        )
        .arg(
            Arg::new("color")
                .short('c')
                .long("color")
                .help("Colorize JSON output")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    let input_file = matches.get_one::<String>("input").unwrap();
    let output_file = matches.get_one::<String>("output");
    let pretty = matches.get_flag("pretty");
    let as_array = matches.get_flag("array");
    let color = matches.get_flag("color");

    convert_avro_to_json_with_color(input_file, output_file, pretty, as_array, color)
        .context("Failed to convert Avro to JSON")?;

    println!("Conversion completed successfully!");
    Ok(())
}

