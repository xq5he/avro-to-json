use apache_avro::Reader;
use anyhow::{Context, Result};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;

pub fn convert_avro_to_json(
    input_file: &str,
    output_file: Option<&String>,
    pretty: bool,
    as_array: bool,
) -> Result<()> {
    convert_avro_to_json_with_color(input_file, output_file, pretty, as_array, false)
}

pub fn convert_avro_to_json_with_color(
    input_file: &str,
    output_file: Option<&String>,
    pretty: bool,
    as_array: bool,
    color: bool,
) -> Result<()> {
    // Open and read the Avro file
    let input_path = Path::new(input_file);
    let file = File::open(input_path)
        .context(format!("Failed to open input file: {}", input_file))?;
    
    let reader = BufReader::new(file);
    let avro_reader = Reader::new(reader)
        .context("Failed to create Avro reader")?;

    // Collect all records
    let mut records = Vec::new();
    
    for record in avro_reader {
        let record = record.context("Failed to read Avro record")?;
        let json_value = avro_value_to_json(&record)?;
        records.push(json_value);
    }

    // Prepare output
    let output: Box<dyn std::io::Write> = if let Some(output_path) = output_file {
        let file = File::create(output_path)
            .context(format!("Failed to create output file: {}", output_path))?;
        Box::new(BufWriter::new(file))
    } else {
        Box::new(std::io::stdout())
    };

    // Write JSON output
    write_json_output(output, records, pretty, as_array, color)
        .context("Failed to write JSON output")?;

    Ok(())
}

pub fn avro_value_to_json(avro_value: &apache_avro::types::Value) -> Result<Value> {
    use apache_avro::types::Value as AvroValue;

    let json_value = match avro_value {
        AvroValue::Null => Value::Null,
        AvroValue::Boolean(b) => Value::Bool(*b),
        AvroValue::Int(i) => Value::Number((*i).into()),
        AvroValue::Long(l) => Value::Number((*l).into()),
        AvroValue::Float(f) => {
            serde_json::Number::from_f64(*f as f64)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        AvroValue::Double(d) => {
            serde_json::Number::from_f64(*d)
                .map(Value::Number)
                .unwrap_or(Value::Null)
        }
        AvroValue::Bytes(bytes) => {
            // Convert bytes to base64 string for JSON representation
            Value::String(base64_encode(bytes))
        }
        AvroValue::String(s) => Value::String(s.clone()),
        AvroValue::Fixed(_, bytes) => {
            // Convert fixed bytes to base64 string
            Value::String(base64_encode(bytes))
        }
        AvroValue::Enum(_, symbol) => Value::String(symbol.clone()),
        AvroValue::Union(_, boxed_value) => avro_value_to_json(boxed_value)?,
        AvroValue::Array(arr) => {
            let mut json_arr = Vec::new();
            for item in arr {
                json_arr.push(avro_value_to_json(item)?);
            }
            Value::Array(json_arr)
        }
        AvroValue::Map(map) => {
            let mut json_obj = serde_json::Map::new();
            for (key, value) in map {
                json_obj.insert(key.clone(), avro_value_to_json(value)?);
            }
            Value::Object(json_obj)
        }
        AvroValue::Record(fields) => {
            let mut json_obj = serde_json::Map::new();
            for (name, value) in fields {
                json_obj.insert(name.clone(), avro_value_to_json(value)?);
            }
            Value::Object(json_obj)
        }
        AvroValue::Date(days) => {
            // Convert days since epoch to ISO date string
            Value::String(format!("days-since-epoch:{}", days))
        }
        AvroValue::TimeMillis(millis) => {
            Value::String(format!("time-millis:{}", millis))
        }
        AvroValue::TimeMicros(micros) => {
            Value::String(format!("time-micros:{}", micros))
        }
        AvroValue::TimestampMillis(millis) => {
            Value::String(format!("timestamp-millis:{}", millis))
        }
        AvroValue::TimestampMicros(micros) => {
            Value::String(format!("timestamp-micros:{}", micros))
        }
        AvroValue::Decimal(decimal) => {
            // Convert decimal to debug string representation
            Value::String(format!("decimal:{:?}", decimal))
        }
        AvroValue::Uuid(uuid) => Value::String(uuid.to_string()),
        AvroValue::Duration(duration) => {
            Value::String(format!("duration:{:?}:{:?}:{:?}", duration.months(), duration.days(), duration.millis()))
        }
        AvroValue::LocalTimestampMillis(millis) => {
            Value::String(format!("local-timestamp-millis:{}", millis))
        }
        AvroValue::LocalTimestampMicros(micros) => {
            Value::String(format!("local-timestamp-micros:{}", micros))
        }
    };

    Ok(json_value)
}

pub fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }
        
        let b0 = buf[0] as usize;
        let b1 = buf[1] as usize;
        let b2 = buf[2] as usize;
        
        result.push(CHARS[b0 >> 2] as char);
        result.push(CHARS[((b0 & 3) << 4) | (b1 >> 4)] as char);
        
        if chunk.len() > 1 {
            result.push(CHARS[((b1 & 15) << 2) | (b2 >> 6)] as char);
        } else {
            result.push('=');
        }
        
        if chunk.len() > 2 {
            result.push(CHARS[b2 & 63] as char);
        } else {
            result.push('=');
        }
    }
    
    result
}

fn colorize_json(json_str: &str) -> String {
    use colored::Colorize;
    
    let mut result = String::new();
    let mut in_string = false;
    let mut escape_next = false;
    let mut chars = json_str.chars().peekable();
    
    while let Some(ch) = chars.next() {
        match ch {
            '"' if !escape_next => {
                in_string = !in_string;
                result.push_str(&ch.to_string().green().to_string());
            }
            '\\' if in_string && !escape_next => {
                escape_next = true;
                result.push_str(&ch.to_string().yellow().to_string());
            }
            _ if in_string => {
                escape_next = false;
                result.push_str(&ch.to_string().green().to_string());
            }
            ':' => {
                result.push_str(&ch.to_string().cyan().to_string());
            }
            ',' => {
                result.push_str(&ch.to_string().white().to_string());
            }
            '{' | '}' | '[' | ']' => {
                result.push_str(&ch.to_string().blue().to_string());
            }
            _ if ch.is_ascii_digit() || ch == '.' || ch == '-' => {
                // Look ahead to get the full number
                let mut number = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_ascii_digit() || next_ch == '.' || next_ch == 'e' || next_ch == 'E' || next_ch == '+' || next_ch == '-' {
                        number.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                result.push_str(&number.yellow().to_string());
            }
            _ if ch.is_alphabetic() => {
                // Handle keywords like true, false, null
                let mut keyword = ch.to_string();
                while let Some(&next_ch) = chars.peek() {
                    if next_ch.is_alphabetic() {
                        keyword.push(chars.next().unwrap());
                    } else {
                        break;
                    }
                }
                match keyword.as_str() {
                    "true" | "false" => result.push_str(&keyword.red().to_string()),
                    "null" => result.push_str(&keyword.purple().to_string()),
                    _ => result.push_str(&keyword),
                }
            }
            _ => {
                result.push(ch);
            }
        }
    }
    
    result
}

pub fn write_json_output(
    mut output: Box<dyn std::io::Write>,
    records: Vec<Value>,
    pretty: bool,
    as_array: bool,
    color: bool,
) -> Result<()> {
    use std::io::{self, IsTerminal, Write};
    
    // Determine if we should use colors (only if outputting to terminal and color is requested)
    let use_color = color && io::stdout().is_terminal();
    
    if as_array {
        // Output as a single JSON array
        let json_array = Value::Array(records);
        let json_str = if pretty {
            serde_json::to_string_pretty(&json_array)?
        } else {
            serde_json::to_string(&json_array)?
        };
        
        if use_color {
            let colored = colorize_json(&json_str);
            writeln!(output, "{}", colored)?;
        } else {
            writeln!(output, "{}", json_str)?;
        }
    } else {
        // Output as newline-delimited JSON (NDJSON)
        for record in records {
            let json_str = if pretty {
                serde_json::to_string_pretty(&record)?
            } else {
                serde_json::to_string(&record)?
            };
            
            if use_color {
                let colored = colorize_json(&json_str);
                writeln!(output, "{}", colored)?;
            } else {
                writeln!(output, "{}", json_str)?;
            }
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use apache_avro::types::Value as AvroValue;

    #[test]
    fn test_avro_null_to_json() {
        let avro_value = AvroValue::Null;
        let json_value = avro_value_to_json(&avro_value).unwrap();
        assert_eq!(json_value, Value::Null);
    }

    #[test]
    fn test_avro_boolean_to_json() {
        let avro_value = AvroValue::Boolean(true);
        let json_value = avro_value_to_json(&avro_value).unwrap();
        assert_eq!(json_value, Value::Bool(true));
    }

    #[test]
    fn test_avro_int_to_json() {
        let avro_value = AvroValue::Int(42);
        let json_value = avro_value_to_json(&avro_value).unwrap();
        assert_eq!(json_value, Value::Number(42.into()));
    }

    #[test]
    fn test_avro_string_to_json() {
        let avro_value = AvroValue::String("hello".to_string());
        let json_value = avro_value_to_json(&avro_value).unwrap();
        assert_eq!(json_value, Value::String("hello".to_string()));
    }

    #[test]
    fn test_avro_array_to_json() {
        let avro_array = vec![
            AvroValue::String("item1".to_string()),
            AvroValue::String("item2".to_string()),
        ];
        let avro_value = AvroValue::Array(avro_array);
        let json_value = avro_value_to_json(&avro_value).unwrap();
        
        match json_value {
            Value::Array(arr) => {
                assert_eq!(arr.len(), 2);
                assert_eq!(arr[0], Value::String("item1".to_string()));
                assert_eq!(arr[1], Value::String("item2".to_string()));
            }
            _ => panic!("Expected array"),
        }
    }

    #[test]
    fn test_avro_record_to_json() {
        let mut fields = Vec::new();
        fields.push(("name".to_string(), AvroValue::String("John".to_string())));
        fields.push(("age".to_string(), AvroValue::Int(30)));
        
        let avro_value = AvroValue::Record(fields);
        let json_value = avro_value_to_json(&avro_value).unwrap();
        
        match json_value {
            Value::Object(obj) => {
                assert_eq!(obj.len(), 2);
                assert_eq!(obj["name"], Value::String("John".to_string()));
                assert_eq!(obj["age"], Value::Number(30.into()));
            }
            _ => panic!("Expected object"),
        }
    }

    #[test]
    fn test_base64_encode() {
        let input = b"hello";
        let encoded = base64_encode(input);
        assert_eq!(encoded, "aGVsbG8=");
        
        let input2 = b"hello world";
        let encoded2 = base64_encode(input2);
        assert_eq!(encoded2, "aGVsbG8gd29ybGQ=");
    }

    #[test]
    fn test_avro_bytes_to_json() {
        let bytes = vec![0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello" in bytes
        let avro_value = AvroValue::Bytes(bytes);
        let json_value = avro_value_to_json(&avro_value).unwrap();
        assert_eq!(json_value, Value::String("SGVsbG8=".to_string()));
    }
}
