#![cfg(feature = "shield")]
extern crate rusoto_core;
extern crate rusoto_shield;

use rusoto_core::Region;
use rusoto_shield::{ListAttacksRequest, Shield, ShieldClient};

#[test]
fn should_list_attacks() {
    let client = ShieldClient::new(Region::UsEast1);
    let request = ListAttacksRequest::default();

    let result = client.list_attacks(request).sync().unwrap();
    println!("{:#?}", result);
}
