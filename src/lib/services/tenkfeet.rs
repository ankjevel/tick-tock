use crate::lib::{
    config::CONFIG,
    models::user::{PagingResponse, User},
};

#[derive(Debug)]
pub enum GetUserErr {
    NoResult,
    Client(String),
    Parse(String),
}

impl GetUserErr {
    pub fn from_query(error: &QueryErr) -> GetUserErr {
        match error {
            QueryErr::Client(string) => GetUserErr::Client(string.to_string()),
            QueryErr::Parse(string) => GetUserErr::Parse(string.to_string()),
        }
    }
}

#[derive(Debug)]
pub enum QueryErr {
    Client(String),
    Parse(String),
}

fn query<T>(url: &str) -> Result<T, QueryErr>
where
    T: for<'de> serde::Deserialize<'de>,
{
    let client = reqwest::blocking::Client::new();

    match client
        .get(&(CONFIG.tenkfeet.root.to_string() + url))
        .header("auth", &CONFIG.tenkfeet.token)
        .send()
    {
        Ok(response) => match response.json::<T>() {
            Ok(response) => Ok(response),
            Err(err) => Err(QueryErr::Parse(err.to_string())),
        },
        Err(err) => Err(QueryErr::Client(err.to_string())),
    }
}

pub fn get_user() -> Result<User, GetUserErr> {
    struct Closure<'s> {
        get: &'s dyn Fn(&Closure, &str) -> Result<User, GetUserErr>,
    }

    let display_name = CONFIG.user.to_string();

    let closure = Closure {
        get: &|closure, url: &str| -> Result<User, GetUserErr> {
            match query::<PagingResponse>(url) {
                Ok(response) => {
                    let mut users = response.clone().data.into_iter();
                    let user = users.find(|user| user.display_name == display_name);
                    if user.is_some() {
                        Ok(user.unwrap())
                    } else if response.paging.next.is_some() {
                        (closure.get)(closure, &response.paging.next.unwrap())
                    } else {
                        Err(GetUserErr::NoResult)
                    }
                }
                Err(error) => Err(GetUserErr::from_query(&error)),
            }
        },
    };

    (closure.get)(&closure, "/api/v1/users?per_page=5")
}
