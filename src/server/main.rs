use tonic::{transport::Server, Request, Response, Status};
use rand::Rng;

mod random_number_service {
    tonic::include_proto!("random_number_service");
}

use random_number_service::{
    random_number_service_server::{RandomNumberService, RandomNumberServiceServer},
    Request as RandomNumberRequest, Response as RandomNumberResponse,
};

#[derive(Default)]
pub struct RandomNumberServiceImpl;
const MAX_NUMBER: i32 = 100;
#[tonic::async_trait]
impl RandomNumberService for RandomNumberServiceImpl {
    async fn get_random(
        &self,
        _request: Request<RandomNumberRequest>,
    ) -> Result<Response<RandomNumberResponse>, Status> {
        let mut rng = rand::thread_rng();
        let response = RandomNumberResponse { number: rng.gen_range(0..MAX_NUMBER) };
        Ok(Response::new(response))
    }

    async fn get_max(
        &self,
        _request: Request<RandomNumberRequest>,
    ) -> Result<Response<RandomNumberResponse>, Status> {
        let response = RandomNumberResponse { number: MAX_NUMBER };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let my_service = RandomNumberServiceImpl::default();

    Server::builder()
        .add_service(RandomNumberServiceServer::new(my_service))
        .serve(addr)
        .await?;

    Ok(())
}
