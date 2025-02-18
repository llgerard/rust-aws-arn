/*!
Provides a set of simple helper functions to make ARNs for the Lambda service.

These resource definitions ae take from the AWS
[documentation](https://docs.aws.amazon.com/IAM/latest/UserGuide/list_awslambda.html#awslambda-resources-for-iam-policies).
*/

use crate::builder::ArnBuilder;
use crate::known::Service::Lambda;
use crate::{Identifier, ResourceIdentifier, ARN};

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// `arn:${Partition}:lambda:${Region}:${Account}:function:${FunctionName}`
///
pub fn function(
    partition: Identifier,
    region: Identifier,
    account: Identifier,
    function_name: Identifier,
) -> ARN {
    ArnBuilder::service_id(Lambda.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("function"),
            function_name,
        ]))
        .into()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}`
///
pub fn layer(
    partition: Identifier,
    region: Identifier,
    account: Identifier,
    layer_name: Identifier,
) -> ARN {
    ArnBuilder::service_id(Lambda.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("layer"),
            layer_name,
        ]))
        .into()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:layer:${LayerName}:${LayerVersion}`
///
pub fn layer_version(
    partition: Identifier,
    region: Identifier,
    account: Identifier,
    layer_name: Identifier,
    layer_version: i32,
) -> ARN {
    ArnBuilder::service_id(Lambda.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("layer"),
            layer_name,
            Identifier::new_unchecked(&layer_version.to_string()),
        ]))
        .into()
}

///
/// `arn:${Partition}:lambda:${Region}:${Account}:event-source-mapping:${UUID}`
///
pub fn event_source_mapping(
    partition: Identifier,
    region: Identifier,
    account: Identifier,
    mapping_uuid: Identifier,
) -> ARN {
    ArnBuilder::service_id(Lambda.into())
        .in_partition_id(partition)
        .in_region_id(region)
        .owned_by(account)
        .is(ResourceIdentifier::from_qualified_id(&[
            Identifier::new_unchecked("event-source-mapping"),
            mapping_uuid,
        ]))
        .into()
}
