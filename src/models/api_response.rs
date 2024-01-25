use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<'a, T: Serialize> {
    pub code: u32,
    pub data: T,
    pub message: &'a str,
}
