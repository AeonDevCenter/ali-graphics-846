use aws_sdk_s3::Client;

#[derive(Clone)]
pub struct AppState {
    s3_client: Client,
    bucket_name: String,
}

impl AppState {
    pub fn new(s3_client: Client, bucket_name: String) -> Self {
        AppState { s3_client, bucket_name}
    }

    pub fn s3_client(&self) -> &Client {
        &self.s3_client
    }

    pub fn bucket_name(&self) -> &String {
        &self.bucket_name
    }
}
