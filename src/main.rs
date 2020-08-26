use hound;
use std::f32::consts::PI;
use std::i16;

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
    let factor = 150;

    // let mut reader = hound::WavReader::open("adio_file4.wav").unwrap();
    let mut reader = hound::WavReader::open("file3.wav").unwrap();
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

    for (i, sample) in reader.samples::<i16>().enumerate() {
        slice_counter += 1;

        // println!("{}", sample.unwrap());

        // rms_accumulator += (sample.unwrap() as i64).pow(2);
        // rms_accumulator += sample.unwrap().pow(2);
        rms_accumulator = sample.unwrap().abs().max(rms_accumulator);

        if slice_counter == new_sample_rate || i == duration as usize {
            // let slice_root_mean = rms_accumulator / slice_counter as i64;
            // let slice_rms = (slice_root_mean as f32).sqrt();

            // let time = i / sample_rate as usize;
            // // println!("{}: {}", time, slice_rms);
            // println!("{},", slice_rms);
            // rms_data.push(slice_rms);

            println!("{},", rms_accumulator);
            rms_data.push(rms_accumulator);

            rms_accumulator = 0;
            slice_counter = 0;
        }
    }

    println!("Data Length: {:?}, {}", rms_data.len(), i16::MAX);
}
