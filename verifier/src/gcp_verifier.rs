use super::gcp_claim::*;
use common::error::{Error, VerificationError};
use common::tee::{verifier::TEEVerifier, TEEType};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Header, Validation};
use log::error;
use reqwest::Client;
use url::Url;
/// This attestation verifier only works for GCP confidential space
#[derive(Debug)]
pub struct GcpVerifier {
    image_reference: String,
    image_digest: String,
    audience: String,
    issuer: String,
}

impl GcpVerifier {
    /// Create a verifier with some golden values.
    /// For debug mode, you can skip verifying image digest since the docker image is under active development.
    pub fn new(
        image_reference: String,
        image_digest: String,
        audience: String,
        issuer: String,
    ) -> Self {
        GcpVerifier {
            image_reference,
            image_digest,
            audience,
            issuer,
        }
    }

    async fn get_openid_configuration(&self) -> Result<OpenIDConfiguration, Error> {
        let url_str = format!("{}{}", EXPECTED_ISSUER, GCP_WELL_KNOWN_URL_PATH);
        let url = Url::parse(&url_str).unwrap();
        let client = Client::builder().build().unwrap();
        match client.get(url.clone()).send().await {
            Ok(res) => {
                let configuration = res.json::<OpenIDConfiguration>().await;
                match configuration {
                    Ok(conf) => {
                        return Ok(conf);
                    }
                    Err(e) => {
                        return Err(Error::SerDeError {
                            message: e.to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                error!("failed to send request: url {}, reason {}", url, e);
                return Err(Error::NetworkError {
                    address: url_str,
                    message: e.to_string(),
                });
            }
        }
    }

    async fn get_json_web_key_sets(&self, url_str: String) -> Result<KeySets, Error> {
        let url = Url::parse(&url_str).unwrap();
        let client = Client::builder().build().unwrap();
        match client.get(url.clone()).send().await {
            Ok(res) => {
                let key_sets = res.json::<KeySets>().await;
                match key_sets {
                    Ok(keys) => {
                        return Ok(keys);
                    }
                    Err(e) => {
                        return Err(Error::SerDeError {
                            message: e.to_string(),
                        })
                    }
                }
            }
            Err(e) => {
                error!("failed to send request: url {}, reason {}", url, e);
                return Err(Error::NetworkError {
                    address: url_str,
                    message: e.to_string(),
                });
            }
        }
    }

    fn find_jwt_key_set(&self, s: &KeySets, kid: String) -> Result<JsonWebKeySet, Error> {
        for k in &s.keys {
            if k.kid == kid {
                return Ok(k.clone());
            }
        }
        return Err(Error::VerificationError {
            teetype: TEEType::GCP,
            error: VerificationError::KidNotFoundError { kid: kid },
        });
    }
}

impl TEEVerifier for GcpVerifier {
    async fn verify(&self, report: String, skip_verify_image_digest: bool) -> Result<bool, Error> {
        let header: Header = decode_header(&report).unwrap();
        if header.alg != Algorithm::RS256 {
            return Err(Error::VerificationError {
                teetype: TEEType::GCP,
                error: VerificationError::SigAlgMismatchError {
                    algorithm: format!("expected RS256 but get {:?}", header.alg),
                },
            });
        }
        let kid = header.kid.unwrap().clone();
        let openid_configuration = self.get_openid_configuration().await?;
        let key_sets: KeySets = self
            .get_json_web_key_sets(openid_configuration.jwks_uri)
            .await?;
        let key_set: JsonWebKeySet = self.find_jwt_key_set(&key_sets, kid)?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[self.audience.clone()]);
        validation.set_issuer(&[self.issuer.clone()]);
        let claims = decode::<Claims>(
            &report,
            &DecodingKey::from_rsa_components(&key_set.n, &key_set.e).unwrap(),
            &validation,
        )
        .map_err(|e| Error::VerificationError {
            teetype: TEEType::GCP,
            error: VerificationError::ValidateTokenError {
                message: e.to_string(),
            },
        })?;

        if claims.claims.submods.container.image_reference != self.image_reference {
            return Err(Error::VerificationError {
                teetype: TEEType::GCP,
                error: VerificationError::GoldenValueMismatchError {
                    value: "image_reference".to_string(),
                    expect: self.image_reference.clone(),
                    get: claims.claims.submods.container.image_reference.clone(),
                },
            });
        }

        if !skip_verify_image_digest {
            if claims.claims.submods.container.image_digest != self.image_digest {
                return Err(Error::VerificationError {
                    teetype: TEEType::GCP,
                    error: VerificationError::GoldenValueMismatchError {
                        value: "image_digest".to_string(),
                        expect: self.image_digest.clone(),
                        get: claims.claims.submods.container.image_digest.clone(),
                    },
                });

                // return Err(Error::GoldenValueMismatchError {
                //     value: "image_digest".to_string(),
                //     expect: self.image_digest.clone(),
                //     get: claims.claims.submods.container.image_digest.clone(),
                // });
            }
        }

        return Ok(true);
    }

    fn get_type(&self) -> Result<TEEType, Error> {
        Ok(TEEType::GCP)
    }
}
