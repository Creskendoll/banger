use cpal::traits::{DeviceTrait, HostTrait};
use cpal::{SampleFormat, SupportedStreamConfig};

fn get_default_input_device(host: &cpal::Host) -> cpal::Device {
    host.default_input_device()
        .expect("no input device available")
}

fn get_default_output_device(host: &cpal::Host) -> cpal::Device {
    host.default_output_device()
        .expect("no output device available")
}

// A function that takes in either a SupportedInputConfigsRange or a SupportedOutputConfigsRange and returns the default config.
fn get_default_config(
    mut configs_range: impl Iterator<Item = cpal::SupportedStreamConfigRange>,
) -> cpal::SupportedStreamConfig {
    configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate()
}

fn get_input_config(device: &cpal::Device) -> cpal::SupportedStreamConfig {
    let input_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");
    get_default_config(input_configs_range)
}

fn get_output_config(device: &cpal::Device) -> cpal::SupportedStreamConfig {
    let output_configs_range = device
        .supported_output_configs()
        .expect("error while querying configs");
    get_default_config(output_configs_range)
}

fn main() {
    let host: cpal::Host = cpal::default_host();

    let input_device: cpal::Device = get_default_input_device(&host);
    println!("Default input device: {}", input_device.name().unwrap());

    let output_device: cpal::Device = get_default_output_device(&host);
    println!("Default output device: {}", output_device.name().unwrap());

    let sup_output_config: SupportedStreamConfig = get_output_config(&output_device);
    let sup_input_config: SupportedStreamConfig = get_input_config(&input_device);

    let err_fn = |err| eprintln!("an error occurred on the output audio stream: {}", err);

    // Create a buffer to write to.
    let mut buffer = vec![];

    let output_config = sup_output_config.config();
    let output_stream = output_device.build_output_stream(
        &output_config,
        move |data: &mut [f32], _info| {
            // Write the buffer to the output stream.
            data = buffer.as_slice().as_mut();
        },
        err_fn,
        None,
    );

    let input_config = sup_input_config.config();
    let input_stream = input_device.build_input_stream(
        &input_config,
        move |data: &[f32], _info| {
            // Write the input stream to the buffer.
            buffer.extend(data);
        },
        err_fn,
        None,
    );

    // Keep the stream alive.
    std::mem::forget(input_stream);
    std::mem::forget(output_stream);

    // Wait for the stream to finish.
    std::thread::park();
}
