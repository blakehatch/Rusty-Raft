// use tonic::{transport::Server, Request, Response, Status};

// #[derive(Default)]
// pub struct YourService;

// // #[tonic::async_trait]
// // impl your_proto_file::your_service_server::YourService for YourService {
// //     async fn your_method(
// //         &self,
// //         request: Request<your_proto_file::RequestType>,
// //     ) -> Result<Response<your_proto_file::ResponseType>, Status> {
// //         // Implement your method logic here
// //         let response = your_proto_file::ResponseType {
// //             // Populate response fields
// //         };

// //         Ok(Response::new(response))
// //     }
// // }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     let addr = "[::1]:50051".parse()?;
//     let service = YourService::default();

//     Server::builder()
//         .add_service(your_proto_file::YourServiceServer::new(service))
//         .serve(addr)
//         .await?;

//     Ok(())
// }


fn main() {

}