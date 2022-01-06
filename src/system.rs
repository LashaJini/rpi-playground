use std::error;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::result;

#[derive(Copy, Clone)]
pub enum Model {
    RaspberryPi4B,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::RaspberryPi4B => write!(f, "Raspberry Pi 4B"),
        }
    }
}

pub struct DeviceInfo {
    model: Model,
}

#[derive(Debug)]
pub enum Error {
    UnknownModel,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UnknownModel => write!(f, "Unknown model."),
        }
    }
}

impl error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

impl DeviceInfo {
    pub fn new() -> Result<DeviceInfo> {
        let model = parse_proc_cpuinfo()?;

        Ok(DeviceInfo { model })
    }

    pub fn get_model(&self) -> Result<Model> {
        Ok(self.model)
    }
}

fn parse_proc_cpuinfo() -> Result<Model> {
    let proc_cpuinfo = BufReader::new(match File::open("/proc/cpuinfo") {
        Ok(file) => file,
        Err(_) => return Err(Error::UnknownModel),
    });

    let mut hardware = String::new();
    let mut revision = String::new();
    for line in proc_cpuinfo.lines().flatten() {
        if let Some(line) = line.strip_prefix("Hardware\t: ") {
            hardware = String::from(line);
        } else if let Some(line) = line.strip_prefix("Revision\t: ") {
            revision = String::from(line);
        }
    }

    println!("Hardware: {}", hardware);
    println!("Revision: {}", revision);

    let model = Model::RaspberryPi4B;
    Ok(model)
}
