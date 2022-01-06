use ::rpi_playground_lib::system::{self, DeviceInfo};

fn main() -> system::Result<()> {
    let model = DeviceInfo::new()?.get_model()?;

    println!("{}", model);

    Ok(())
}
