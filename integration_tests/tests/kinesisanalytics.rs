#![cfg(feature = "kinesisanalytics")]

extern crate rusoto_core;
extern crate rusoto_kinesisanalytics;

use rusoto_core::Region;
use rusoto_kinesisanalytics::{KinesisAnalytics, KinesisAnalyticsClient, ListApplicationsRequest};

#[test]
fn should_list_applications() {
    let client = KinesisAnalyticsClient::new(Region::UsEast1);
    let request = ListApplicationsRequest::default();

    let result = client.list_applications(request).sync().unwrap();
    println!("{:#?}", result);
}
