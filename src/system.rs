use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result;

#[derive(Debug)]
pub enum Error {
    UknownModel,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::UknownModel => write!(f, "Uknown Raspberry PI model."),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Model {
    RaspberryPi4B,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Model::RaspberryPi4B => write!(f, "Raspberry PI 4B"),
        }
    }
}

pub struct DeviceInfo {
    model: Model,
}

impl DeviceInfo {
    pub fn new() -> Result<DeviceInfo> {
        let model = parse_proc_cpuinfo()?;

        Ok(DeviceInfo { model })
    }

    pub fn get_model(&self) -> Model {
        self.model
    }
}

pub fn parse_proc_cpuinfo() -> Result<Model> {
    let proc_cpuinfo = BufReader::new(match File::open("/proc/cpuinfo") {
        Ok(file) => file,
        Err(_) => return Err(Error::UknownModel),
    });

    let mut hardware = String::new();
    let mut revision = String::new();
    for line in proc_cpuinfo.lines().flatten() {
        if let Some(line) = line.strip_prefix("Hardware\t: ") {
            hardware = String::from(line);
        } else if let Some(line_value) = line.strip_prefix("Revision\t: ") {
            revision = String::from(line_value).to_lowercase();
        }
    }

    println!("Hardware is {}", &hardware);
    println!("Revision is {}", &revision);

    let model = Model::RaspberryPi4B;
    Ok(model)
}
