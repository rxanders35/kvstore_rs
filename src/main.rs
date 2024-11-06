use proto::*;
use tonic::{transport::Server, Request, Response, Status};

pub mod proto {
    tonic::include_proto!("kv");
}


#[derive(Debug, Default)]
struct KVService {}

impl KVstore for KVService {
   async fn put(
    &self, 
    request: Request<PutRq>
    ) -> Result<Response<PutResp>, Status> {
        // TODO: Implement 
   } 
}

fn main() {
    println!("Hello, world!");
}
