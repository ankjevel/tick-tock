use crate::lib::{config::CONFIG, APP_NAME_ID};
use directories::BaseDirs;
use md5;
use std::{
    fs::{create_dir, File},
    io,
    path::{Path, PathBuf},
};

lazy_static! {
    static ref DATA_DIR: String = {
        let base_dirs = BaseDirs::new().unwrap();
        let mut data_dir = base_dirs.data_dir().to_owned();
        data_dir.push(APP_NAME_ID.to_string());
        String::from(data_dir.to_str().unwrap())
    };
}

#[derive(Debug)]
pub enum GetErr {
    Client(String),
    Parse(String),
}

pub fn get_file(url: &str) -> Result<String, String> {
    let file_as_md5 = md5::compute(url);
    let data_dir = DATA_DIR.as_str();

    let mut path = PathBuf::new();
    path.push(data_dir);
    path.push(format!("{:x}", file_as_md5));

    if CONFIG.debug {
        println!("{:?}", path);
    }

    let path_string = String::from(path.to_str().unwrap());

    if path.exists() {
        return Ok(path_string);
    }

    match download_file(url, &path_string.as_str()) {
        Ok(()) => Ok(path_string),
        Err(error) => Err(error.to_string()),
    }
}

fn download_file<'a>(url: &str, path: &str) -> Result<(), String> {
    let client = reqwest::blocking::Client::new();

    match client.get(url).send() {
        Ok(response) => {
            let mut response = response;
            let mut out = File::create(path).expect(&"failed to create file".to_string());
            io::copy(&mut response, &mut out).expect(&"failed to copy content".to_string());
            Ok(())
        }
        Err(err) => Err(err.to_string()),
    }
}

pub fn get_text(url: &str) -> Result<String, GetErr> {
    let client = reqwest::blocking::Client::new();

    match client.get(url).send() {
        Ok(response) => match response.text() {
            Ok(response) => Ok(response),
            Err(err) => Err(GetErr::Parse(err.to_string())),
        },
        Err(err) => Err(GetErr::Client(err.to_string())),
    }
}

pub fn setup_data_dir() -> io::Result<()> {
    let data_dir = DATA_DIR.as_str();
    let path = Path::new(&data_dir);

    create_dir(path)?;

    Ok(())
}
