use std::path::PathBuf;

use async_trait::async_trait;

#[async_trait]
pub trait RetryMove: Send + Sync {
    fn get_base_directory(&self) -> String;
    async fn retry_move(&self, pathbuf: PathBuf);
}

struct S3Uploader {
    base_directory: String,
    default_bucket: String,
    bucket_map: Vec<(String, String)>,
}

impl S3Uploader {
    pub fn new(pathbuf: PathBuf, default_bucket: String, mut map: Vec<(String, String)>) -> Self {
        let base = pathbuf
            .to_str()
            .expect("converson of pathbuf to string failed!")
            .to_owned();
    }
}
