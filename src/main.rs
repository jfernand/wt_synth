use rodio::Source;
use std::f32::consts::PI;
use std::time::Duration;

struct WaveTableOscillator {
    sample_rate: u32,
    wave_table: Vec<f32>,
    index: f32,
    index_increment: f32,
}

impl WaveTableOscillator {
    fn new(sample_rate: u32, wave_table: Vec<f32>) -> Self {
        WaveTableOscillator {
            sample_rate,
            wave_table,
            index: 0.,
            index_increment: 0.,
        }
    }
    fn set_frequency(&mut self, frequency: f32) {
        self.index_increment = frequency * self.wave_table.len() as f32 / self.sample_rate as f32;
    }

    fn get_sample(&mut self) -> f32 {
        let sample = self.lerp();
        self.index += self.index_increment;
        self.index %= self.wave_table.len() as f32;
        sample
    }

    fn lerp(&self) -> f32 {
        let truncated_index = self.index as usize;
        let next_index = (truncated_index + 1) % self.wave_table.len();

        let next_index_weight = self.index - truncated_index as f32;
        let truncated_index_weight = 1.0 - next_index_weight;

        truncated_index_weight * self.wave_table[truncated_index]
            + next_index_weight * self.wave_table[next_index]
    }
}

impl Iterator for WaveTableOscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.get_sample())
    }
}

impl Source for WaveTableOscillator {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        None
    }
}

fn main() {
    let sine_wave_table = square_wave();
    let mut sine_oscillator = WaveTableOscillator::new(44100, sine_wave_table);
    sine_oscillator.set_frequency(220.0);

    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let _result = stream_handle.play_raw(sine_oscillator.convert_samples());
    std::thread::sleep(Duration::from_secs(5));
    println!("Hello, world!");
}

fn sine_wave() -> Vec<f32> {
    let wave_table_size = 64;
    let mut sine_wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        sine_wave_table.push({ 2. * PI * n as f32 / wave_table_size as f32 }.sin());
    }
    sine_wave_table
}

fn square_wave() -> Vec<f32> {
    let wave_table_size = 64;
    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);
    for n in 0..wave_table_size {
        if n > wave_table_size / 2 {
            wave_table.push(1.0);
        } else {
            wave_table.push(-1.0);
        }
    }
    wave_table
}
