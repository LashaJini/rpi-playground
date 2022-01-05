use ::rpi_playground_lib::system::{DeviceInfo, Result};

fn main() -> Result<()> {
    let model = DeviceInfo::new()?.get_model();
    println!("Model: {}", model);

    Ok(())
}
