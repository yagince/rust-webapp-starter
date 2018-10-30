use jwt::{self, Header, Algorithm, errors::Result, Validation, TokenData};

const KEY: &str = "secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub user_id: String,
}

pub fn encode(claim: &Claims) -> Result<String> {
    jwt::encode(&Header::default(), claim, KEY.as_ref())
}

pub fn decode(token: &String) -> Result<TokenData<Claims>> {
    let validation = Validation {
        validate_exp: false,
        ..Default::default()
    };
    jwt::decode(token, KEY.as_ref(), &validation)
}
