#[derive(Deserialize)]
pub struct AuthResponseDto {
    pub session: SessionDto
}

#[derive(Deserialize)]
pub struct SessionDto {
    pub key: String
}
