pub mod gcp_claim;
pub mod gcp_verifier;

#[cfg(test)]
mod tests {
    use super::*;
    use common::tee::verifier::TEEVerifier;
    use gcp_verifier::GcpVerifier;
    #[tokio::test]
    async fn verify_gcp_token() {
        let token = "eyJhbGciOiJSUzI1NiIsImtpZCI6IjFjNjdmYWVhYjRhYzE1ZDJmNmZmODMwY2E2ZmM1N2YyYmVhM2Y0YmIiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOiJodHRwczovL3N0cy5nb29nbGVhcGlzLmNvbSIsImV4cCI6MTcxNjEwNjYyOSwiaWF0IjoxNzE2MTAzMDI5LCJpc3MiOiJodHRwczovL2NvbmZpZGVudGlhbGNvbXB1dGluZy5nb29nbGVhcGlzLmNvbSIsIm5iZiI6MTcxNjEwMzAyOSwic3ViIjoiaHR0cHM6Ly93d3cuZ29vZ2xlYXBpcy5jb20vY29tcHV0ZS92MS9wcm9qZWN0cy9iaW9uaWMtbWVyY3VyeS00MjE4MDkvem9uZXMvdXMtd2VzdDEtYi9pbnN0YW5jZXMvaW5mZXJlbmNlLWRlbW8iLCJzZWNib290Ijp0cnVlLCJvZW1pZCI6MTExMjksImh3bW9kZWwiOiJHQ1BfQU1EX1NFViIsInN3bmFtZSI6IkNPTkZJREVOVElBTF9TUEFDRSIsInN3dmVyc2lvbiI6WyIyNDA0MDIiXSwiZGJnc3RhdCI6ImVuYWJsZWQiLCJzdWJtb2RzIjp7ImNvbmZpZGVudGlhbF9zcGFjZSI6eyJtb25pdG9yaW5nX2VuYWJsZWQiOnsibWVtb3J5IjpmYWxzZX19LCJjb250YWluZXIiOnsiaW1hZ2VfcmVmZXJlbmNlIjoiYXNpYS1kb2NrZXIucGtnLmRldi9iaW9uaWMtbWVyY3VyeS00MjE4MDkvYWl6ZWwvYWl6ZWxfaW5mZXJlbmNlOjAuMS4wIiwiaW1hZ2VfZGlnZXN0Ijoic2hhMjU2OjAxMDcwYTg1OGRlN2Y0ODc0YjNiYmY2OWMwYWI0Mjg2MjM0MmIzOWQ1YzFiNDhjN2VmNTVmYjg0MjMxZjhkMzEiLCJyZXN0YXJ0X3BvbGljeSI6Ik5ldmVyIiwiaW1hZ2VfaWQiOiJzaGEyNTY6MTBmOTc0MWIxMGQ2OThkOWVkYWU2Y2MzZmNmMWJhNWNjOGYxMGFiNDRiMzNmMzkyZTY4Y2I5MDM4YTliZGI5NyIsImVudiI6eyJIT1NUTkFNRSI6ImluZmVyZW5jZS1kZW1vIiwiUEFUSCI6Ii91c3IvbG9jYWwvc2JpbjovdXNyL2xvY2FsL2JpbjovdXNyL3NiaW46L3Vzci9iaW46L3NiaW46L2JpbiJ9LCJhcmdzIjpbIi9iaW4vc2giLCItYyIsIi9iaW4vYmFzaCBib290c3RyYXAuc2giXX0sImdjZSI6eyJ6b25lIjoidXMtd2VzdDEtYiIsInByb2plY3RfaWQiOiJiaW9uaWMtbWVyY3VyeS00MjE4MDkiLCJwcm9qZWN0X251bWJlciI6Ijk5MTQ0OTYyOTQzNCIsImluc3RhbmNlX25hbWUiOiJpbmZlcmVuY2UtZGVtbyIsImluc3RhbmNlX2lkIjoiMjM4NTYyMTg5MjM5OTkwMDA3MiJ9fSwiZ29vZ2xlX3NlcnZpY2VfYWNjb3VudHMiOlsiOTkxNDQ5NjI5NDM0LWNvbXB1dGVAZGV2ZWxvcGVyLmdzZXJ2aWNlYWNjb3VudC5jb20iXX0.pWaP2tfhSSbw7LRTk9VLCh2XhwuNaFT_8LwoayjwvyP6YtoiZ9vjFazmo_oDeit2DEDcyiS5ZxTKbjytUNPuBN0zHGjcBIiUWhdAMIshBtel7Ron424qn955HEfCIVD4ATGy8UWFiCEskCB_Akv05AQOUmH9YyxLHQ-b2xmak1EgJnNg2a4ISKGowwTM_B778-HHdw59npMmYdaes6FVSH4l3toy2hgMjkDmzkJOdBuAUb-GSRFaIyH7_gFpVReWuYfc-GyXrsczoblLBDSO9XwpkY1GDOjcU_3axSBYDfJzMn2mrY4MFRU4XjxQ0YJaEZ8j-Jq097JSISD0hDn4UA";
        let image_reference =
            "asia-docker.pkg.dev/bionic-mercury-421809/aizel/aizel_inference:0.1.0";
        let image_digest =
            "sha256:01070a858de7f4874b3bbf69c0ab42862342b39d5c1b48c7ef55fb84231f8d31";
        let audience = "https://sts.googleapis.com";
        let issuer = "https://confidentialcomputing.googleapis.com";
        let gcp_verifier = GcpVerifier::new(
            image_reference.to_string(),
            image_digest.to_string(),
            audience.to_string(),
            issuer.to_string(),
        );

        gcp_verifier.verify(token.to_string(), true).await.unwrap();
    }
}
