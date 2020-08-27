use hound;
use serde_json::json;
use std::f32::consts::PI;
use std::fs::File;
use std::i16;
use std::io::prelude::*;
use std::path::Path;

#[allow(dead_code)]
fn write_basic_wav() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create("sine.wav", spec).unwrap();
    for t in (0..44100).map(|x| x as f32 / 44100.0) {
        let sample = (t * 440.0 * 2.0 * PI).sin();
        let amplitude = i16::MAX as f32;
        writer.write_sample((sample * amplitude) as i16).unwrap();
    }
    writer.finalize().unwrap();
}

fn main() {
    let factor = 30;
    let max = i16::MAX;
    let min = i16::MIN;

    // let mut reader = hound::WavReader::open("adio_file4.wav").unwrap();
    let mut reader = hound::WavReader::open("file2.wav").unwrap();
    // let mut reader = hound::WavReader::open("sine.wav").unwrap();

    println!("Spec: {:?}", reader.spec());
    let sample_count = reader.duration();
    let sample_rate = reader.spec().sample_rate;
    let duration = sample_count as f32 / reader.spec().sample_rate as f32;
    let new_sample_rate = reader.spec().sample_rate / factor;

    println!("Sample Count: {}", sample_count);
    println!("File Duration: {}", duration);
    println!("New Sample Rate: {}", new_sample_rate);

    let mut slice_counter = 0;
    // let mut rms_accumulator: i64 = 0;
    let mut rms_accumulator: i16 = 0;

    let mut rms_data = vec![];

    let mut sample_max = 0;
    let mut sample_min = 0;

    for (i, sample) in reader.samples::<i16>().enumerate() {
        slice_counter += 1;

        let sample_val = sample.unwrap();

        if sample_val > max {
            sample_max = max;
        }

        if sample_val > sample_max {
            sample_max = sample_val;
        }

        if sample_val < min {
            sample_min = min;
        }

        if sample_val < sample_min {
            sample_min = sample_val;
        }

        if slice_counter == new_sample_rate || i == duration as usize {
            let time = i / sample_rate as usize;

            // println!("{}: Max {}, Min {}", time, sample_max, sample_min);
            rms_data.push(sample_min);
            rms_data.push(sample_max);

            sample_min = 0;
            sample_max = 0;

            rms_accumulator = 0;
            slice_counter = 0;
        }
    }

    println!("Data Length: {:?}", rms_data.len());
    let wav_data = json!({
        "duration": duration,
        "orig_sample_count": sample_count,
        "new_sample_rate": new_sample_rate,
        "sample_rate": sample_rate,
        "new_sample_count": rms_data.len(),
        "min": min,
        "max": max,
        "rms": rms_data
    });

    println!("{}", wav_data);

    let path = Path::new("data.json");

    let mut file = match File::create("data.json") {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why),
        Ok(file) => file,
    };

    match file.write_all(wav_data.to_string().as_bytes()) {
        Err(why) => panic!("Couldn't write to {}: {}", path.display(), why),
        Ok(_) => println!("Successfully wrote to {}", path.display()),
    };
}
