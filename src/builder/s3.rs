/*!
Provides a set of simple helper functions to make ARNs for the S3 service.

These resource definitions ae take from the AWS
[documentation]( https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html#amazons3-resources-for-iam-policies)
*/

use crate::builder::{ArnBuilder, ResourceBuilder};
use crate::{Resource, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// The service name portion of the ARN.
///
pub const SERVICE_NAME: &str = "s3";

///
/// `arn:${Partition}:s3:::${BucketName}`
///
pub fn bucket_in(partition: &str, bucket_name: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .is(ResourceBuilder::new(bucket_name).build())
        .build()
}

///
/// `arn:aws:s3:::${BucketName}`
///
pub fn bucket(bucket_name: &str) -> ARN {
    bucket_in("aws", bucket_name)
}

///
/// `arn:${Partition}:s3:::${BucketName}/${ObjectName}`
///
pub fn object_in(partition: &str, bucket_name: &str, object_name: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .is(ResourceBuilder::new(&format!("{}/{}", bucket_name, object_name)).build())
        .build()
}

///
/// `arn:$aws:s3:::${BucketName}/${ObjectName}`
///
pub fn object(bucket_name: &str, object_name: &str) -> ARN {
    object_in("aws", bucket_name, object_name)
}

///
/// `arn:$aws:s3:::${BucketName}/${ObjectName}`
///
/// This function will panic if `bucket` is not an ARN for an S3 bucket.
///
pub fn object_from(bucket: &ARN, object_name: &str) -> ARN {
    if bucket.service != SERVICE_NAME.to_string() {
        panic!("You can't make an S3 object from a {} ARN.", bucket.service);
    }
    if let Resource::Id(id) = &bucket.resource {
        ARN {
            resource: ResourceBuilder::new(&format!("{}/{}", id, object_name)).build(),
            ..bucket.clone()
        }
    } else {
        panic!("S3 bucket resources *must* be identifiers.");
    }
}

///
/// `arn:${Partition}:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job_in(partition: &str, region: &str, account: &str, job_id: &str) -> ARN {
    ArnBuilder::new(SERVICE_NAME)
        .in_partition(partition)
        .in_region(region)
        .owned_by(account)
        .is(ResourceBuilder::new(job_id).build())
        .build()
}

///
/// `arn:aws:s3:${Region}:${Account}:job/${JobId}`
///
pub fn job(region: &str, account: &str, job_id: &str) -> ARN {
    job_in("aws", region, account, job_id)
}
