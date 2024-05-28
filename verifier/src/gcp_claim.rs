use serde::{Deserialize, Serialize};

pub const EXPECTED_ISSUER: &str = "https://confidentialcomputing.googleapis.com";
pub const GCP_WELL_KNOWN_URL_PATH: &str = "/.well-known/openid-configuration";

#[derive(Debug, Serialize, Deserialize)]
pub struct ContainerClaims {
    pub image_reference: String,
    pub image_digest: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Submods {
    pub container: ContainerClaims,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub aud: String,
    pub iss: String,
    pub sub: String,
    pub exp: usize,
    pub submods: Submods,
    pub hwmodel: String,
    pub swname: String,
    pub swversion: Vec<String>,
    pub secboot: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct JsonWebKeySet {
    pub alg: String,
    pub kty: String,
    pub n: String,
    #[serde(rename = "use")]
    pub usage: String,
    pub kid: String,
    pub e: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeySets {
    pub keys: Vec<JsonWebKeySet>,
}

#[derive(Debug, Deserialize)]
pub struct OpenIDConfiguration {
    pub issuer: String,
    pub jwks_uri: String,
}
