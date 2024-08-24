use std::sync::Arc;
use futures::lock::Mutex;
use tonic::transport::Channel;
use warp::Filter;
use warp::http::StatusCode;

mod random_number_service {
    tonic::include_proto!("random_number_service");
}
use random_number_service::{random_number_service_client::RandomNumberServiceClient, Request};

#[tokio::main]
async fn main() {
    // TODO: Use proto annotations to map rest api and grpc methods. Currently protoc does not support this.
    let grpc_client = Arc::new(Mutex::new(RandomNumberServiceClient::connect("http://[::1]:50051").await.unwrap()));
    
    let get_random = warp::path!("get_random")
        .and(with_grpc_client(Arc::clone(&grpc_client)))
        .and_then(handle_get_random);

    let get_max = warp::path!("get_max")
        .and(with_grpc_client(Arc::clone(&grpc_client)))
        .and_then(handle_get_max);

    let api = get_random.or(get_max);

    const PORT: u16 = 3030;
    println!("Serving on port {}", PORT);
    println!("Try http://localhost:{}/get_random", PORT);
    warp::serve(api).run(([127, 0, 0, 1], PORT)).await;
}

fn with_grpc_client(client: Arc<Mutex<RandomNumberServiceClient<Channel>>>) -> impl Filter<Extract = (Arc<Mutex<RandomNumberServiceClient<Channel>>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || Arc::clone(&client))
}

async fn handle_get_random(client: Arc<Mutex<RandomNumberServiceClient<Channel>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut client = client.lock().await;
    let resp = client.get_random(Request{}).await.unwrap();
    let number = resp.into_inner().number;
    Ok(warp::reply::with_status(warp::reply::json(&number), StatusCode::OK))
}

async fn handle_get_max(client: Arc<Mutex<RandomNumberServiceClient<Channel>>>) -> Result<impl warp::Reply, warp::Rejection> {
    let mut client = client.lock().await;
    let resp = client.get_max(Request{}).await.unwrap();
    let number = resp.into_inner().number;
    Ok(warp::reply::with_status(warp::reply::json(&number), StatusCode::OK))
}