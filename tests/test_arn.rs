use aws_arn::{Identifier, ResourceIdentifier, ARN};
use std::str::FromStr;

fn parse_and_compare(test_arn: &str, expected: ARN) {
    let result = ARN::from_str(test_arn);
    assert!(result.is_ok());
    let arn = result.unwrap();
    assert_eq!(arn, expected);
}

#[test]
fn test_valid_arn_to_string() {
    let arn = ARN {
        partition: None,
        service: Identifier::new_unchecked("s3"),
        region: None,
        account_id: None,
        resource: ResourceIdentifier::new_unchecked("mythings/athing"),
    };
    assert_eq!(arn.to_string(), "arn:aws:s3:::mythings/athing");
}

#[test]
fn test_arn_from_valid_str() {
    parse_and_compare(
        "arn:aws:s3:us-east-1:123456789012:job/23476",
        ARN {
            partition: Some(Identifier::new_unchecked("aws")),
            service: Identifier::new_unchecked("s3"),
            region: Some(Identifier::new_unchecked("us-east-1")),
            account_id: Some(Identifier::new_unchecked("123456789012")),
            resource: ResourceIdentifier::new_unchecked("job/23476"),
        },
    );
}

#[test]
fn test_github_issues_2() {
    let result =
        ARN::from_str("arn:aws:cloudwatch:us-west-2:123456789012:alarm:Production:LB:High4xx");
    assert!(result.is_ok());
    let arn = result.unwrap();
    assert_eq!(arn.partition, Some(Identifier::new_unchecked("aws")));
    assert_eq!(arn.service, Identifier::new_unchecked("cloudwatch"));
    assert_eq!(arn.region, Some(Identifier::new_unchecked("us-west-2")));
    assert_eq!(
        arn.account_id,
        Some(Identifier::new_unchecked("123456789012"))
    );
    assert_eq!(
        arn.resource,
        ResourceIdentifier::new_unchecked("alarm:Production:LB:High4xx")
    );
    assert!(arn.resource.contains_qualified());
}
