use base64::decode;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

/// JWT represents a JSON web token
#[derive(Serialize, Deserialize, Debug)]
pub struct JWT {
    pub header: HashMap<String, String>,
    pub body: HashMap<String, serde_json::Value>,
    pub signature: String,
    token: String,
}

pub type JWTError = Box<dyn std::error::Error>;
pub type JWTResult = Result<JWT, JWTError>;

impl JWT {
    /// Return a new JWT for a given token string
    ///
    /// ```
    /// let token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJsc2t5d2Fsa2VyIiwiaWF0IjoyMzMzNjY0MDB9.k-tTF2CIZ-vu6-syRnCw3Zlc4jwfBCXAQRAyk0mtmso";
    /// let result = jwtdecode::jwt::JWT::new(token.to_string()).unwrap();
    /// assert_eq!(result.body.get("sub").unwrap(), "lskywalker");
    /// ```
    pub fn new(input_string: String) -> JWTResult {
        let parts: Vec<&str> = input_string.splitn(3, '.').collect::<Vec<&str>>();
        if parts.len() != 3 {
            return Err(JWTError::from("Not enough parts for a valid jwt"));
        }

        let decoded_header = decode(parts[0])?;
        let header = serde_json::from_slice(&decoded_header)?;

        let decoded_body = decode(parts[1])?;
        let body = serde_json::from_slice(&decoded_body)?;

        Ok(JWT {
            header: header,
            body: body,
            signature: String::from(parts[2]),
            token: input_string,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::jwt::JWT;

    #[test]
    fn jwt_new_valid() {
        let valid_token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
        let result = JWT::new(valid_token.to_string()).unwrap();

        assert_eq!(result.header.get("typ").unwrap(), "JWT");
        assert_eq!(result.header.get("alg").unwrap(), "HS256");
        assert_eq!(result.body.get("sub").unwrap(), "1234567890");
    }
}
