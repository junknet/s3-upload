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
        map.sort_by(|a, b| a.0.cmp(&b.0));
        Self {
            base_directory: base,
            default_bucket,
            bucket_map: map,
        }
    }
}

fn get_bucket_and_prefix<'a>(pathbuf: &'a PathBuf, uploader: &'a S3Uploader) -> (&'a str, String) {
    let pb = pathbuf.strip_prefix(&uploader.base_directory).unwrap();
    for (base, bucket) in uploader.bucket_map.iter() {
        if pb.starts_with(base) {
            let pbs = pb
                .strip_prefix(base)
                .expect("unable to strip the prefix")
                .to_string_lossy()
                .to_string();
            return (bucket, pbs);
        }
    }
    let pb = pb.to_string_lossy().to_string();
    (&uploader.base_directory, pb)
}

#[cfg(test)]

mod test {
    use std::path::PathBuf;

    use super::{get_bucket_and_prefix, S3Uploader};

    #[test]
    fn test_get_prefx() {
        let mut map = Vec::<(String, String)>::new();
        map.push(("subdir".to_string(), "123".to_string()));
        let base_path = PathBuf::from("/tmp/uploader");
        let file_path = PathBuf::from("/tmp/uploader/subdir/jedi/123.txt");
        let uploader = S3Uploader::new(base_path, "test_bucket".to_string(), map);
        let (bucket, prefix) = get_bucket_and_prefix(&file_path, &uploader);
        println!("bucket: {}, prefix: {}", bucket, prefix)
    }
}
