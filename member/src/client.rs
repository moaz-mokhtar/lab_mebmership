// use members::membership_client::MembershipClient;
// use members::AddMemberRequest;

// pub mod members {
//     tonic::include_proto!("members");
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // utils::initiate_logging();
//     println!("Membership service started");


    

//     /*
//     *Algorthm:*
//         - start server to listen to new membership requests
//         - when receive new membership request:
//             - connect to the db service.
//                 - check new member in db and return feedback.
//             - If new member to be added, request add new member from db and return feedback.
//     */  

//     let mut client = MembershipClient::connect("http://[::1]:50051").await?;

//     let request = tonic::Request::new(AddMemberRequest {
//         name: "Ahmed".to_owned(),
//     });

//     let response = client.add_member(request).await?;

//     println!("Membership RESPONSE = {:?}", response);

//     Ok(())
// }
