use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::{Client, Error, Region, PKG_VERSION};

/// Lists your Amazon S3 buckets in the Region.
/// # Arguments
///
/// * `[-r REGION]` - The Region in which the client is created. If not
///   supplied, uses the value of the **AWS_REGION** environment variable. If
///   the environment variable is not set, defaults to **us-west-2**.
/// * `[-v]` - Whether to display additional information.
#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let region_provider =
        RegionProviderChain::first_try(Region::new("us-east-1")).or_default_provider();
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    println!();

    println!("S3 client version: {}", PKG_VERSION);
    println!("Region:            {}", shared_config.region().unwrap());
    println!();

    let resp = client.list_buckets().send().await?;
    let buckets = resp.buckets().unwrap_or_default();
    let num_buckets = buckets.len();

    for bucket in buckets {
        println!("{:?}", bucket);
    }

    println!();
    println!("Found {} buckets", num_buckets);

    Ok(())
}
