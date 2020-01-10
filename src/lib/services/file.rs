#[derive(Debug)]
pub enum GetErr {
    Client(String),
    Parse(String),
}

#[derive(Debug)]
pub enum FileErr {
    Client(String),
    Parse(String),
}

pub fn get_file(_url: &str) -> Result<String, FileErr> {
    Ok("/home/dpn/Downloads/790d0dc8-49c8-4403-b84b-ee2be7ff2bad.jpeg".to_string())
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
