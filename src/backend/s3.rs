use std::error::Error;

use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::Region;
use std::env;
use crate::backend::base::Backend;

#[derive(Clone)]
pub struct S3Backend {
    bucket: Bucket,
}

impl Backend for S3Backend {
    fn list_files(&self) -> Result<Vec<String>, std::io::Error> {
        println!("list_files on bucket {}", self.bucket.name.as_str());
        todo!()
    }

    fn get_file_contents(&self, _name: &str) -> Result<Vec<u8>, std::io::Error> {
        todo!()
    }
}

pub fn create() -> Result<Box<dyn Backend>, Box<dyn Error>> {
    let env_bucket = env::var("YEENSERVE_S3_BUCKET")?;
    let env_region = env::var("YEENSERVE_S3_REGION")?;
    let env_endpoint = env::var("YEENSERVE_S3_ENDPOINT")?;

    let region = Region::Custom {
        region: env_region,
        endpoint: env_endpoint,
    };
    let creds = Credentials::from_env_specific(
        Some("YEENSERVE_S3_ACCESS_KEY"),
        Some("YEENSERVE_S3_SECRET_KEY"),
        None,
        None,
    ).unwrap();
    let bucket = Bucket::new(&*env_bucket, region, creds)?;

    return Ok(Box::from(S3Backend {
        bucket
    }));
}
