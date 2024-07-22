use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn ip_and_mask_to_cidr(ip_and_mask: &str) -> Option<String> {
    let parts: Vec<&str> = ip_and_mask.split_whitespace().collect();
    if parts.len() != 2 {
        return None;
    }
    let ip = parts[0];
    let subnet_mask = parts[1];

    let mask_parts: Vec<u8> = subnet_mask
        .split('.')
        .map(|part| u8::from_str_radix(part, 10).ok())
        .collect::<Option<Vec<u8>>>()?;

    let mut count = 0;
    for part in mask_parts {
        let binary = format!("{:08b}", part);
        for bit in binary.chars() {
            if bit == '1' {
                count += 1;
            }
        }
    }

    Some(format!("{}/{}", ip, count))
}

fn batch_convert_and_write(input_file_path: &str, output_file_path: &str) {
    let input_file = File::open(input_file_path).unwrap();
    let reader = BufReader::new(input_file);

    let mut output_file = BufWriter::new(File::create(output_file_path).unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        if let Some(cidr) = ip_and_mask_to_cidr(&line) {
            writeln!(output_file, "{}", cidr).unwrap();
        }
    }
}

fn main() {
    let input_file_path = "D:/vscodecoding/Rustworkspace/SubnetConverter/src/ak_overwatch.txt";
    let output_file_path = "output.txt";
    batch_convert_and_write(input_file_path, output_file_path);
}
