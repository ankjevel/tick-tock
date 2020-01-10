#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: u64,
    pub display_name: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub user_type_id: u64,
    pub billable: bool,
    pub hire_date: Option<String>,
    pub termination_date: Option<String>,
    pub mobile_phone: Option<String>,
    pub office_phone: Option<String>,
    pub archived: bool,
    pub archived_at: Option<String>,
    pub deleted: bool,
    pub deleted_at: Option<String>,
    pub account_owner: bool,
    pub invitation_pending: bool,
    pub user_settings: u64,
    pub guid: String,
    pub employee_number: Option<String>,
    pub billability_target: f64,
    pub billrate: f64,
    pub role: String,
    pub discipline: String,
    pub location: String,
    pub r#type: String,
    pub has_login: bool,
    pub login_type: String,
    pub license_type: String,
    pub thumbnail: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Paging {
    pub per_page: u64,
    pub page: u64,
    pub previous: Option<String>,
    pub next: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PagingResponse {
    pub paging: Paging,
    pub data: Vec<User>,
}
