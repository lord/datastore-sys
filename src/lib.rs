extern crate protobuf;
extern crate grpc;
extern crate futures;
extern crate futures_cpupool;

pub mod datastore;
pub mod datastore_grpc;
pub mod entity;
pub mod query;
pub mod wrappers;
pub mod latlng;
pub mod timestamp;
pub mod struct_pb;

#[test]
fn it_works() {
}
