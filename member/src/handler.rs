use actix_web::{
    get, post,
    web::{self, ServiceConfig},
    HttpResponse, Responder,
};
use log::info;

use members::membership_client::MembershipClient;
use members::AddMemberRequest;
use uuid::Uuid;

use crate::{
    entity::{Member, MemberRequest},
    handler::members::{LicenseMemberRequest, PrintCardRequest, PrintLicenseRequest},
};

pub mod members {
    tonic::include_proto!("members");
}

pub fn routes_config(config: &mut ServiceConfig) {
    config.service(health).service(register);
}

#[get("/")]
/// Route to test API functionality without any communcation with DB.
pub async fn health() -> impl Responder {
    HttpResponse::Ok().body("Healthy")
}

#[post("/register")]
pub async fn register(
    data: web::Json<MemberRequest>,
) -> Result<HttpResponse, Box<dyn std::error::Error>> {
    info!("/register -> pram of `register`: {:?}", data.0.clone());
    let empty = format!("");
    let mut member: Member = Member {
        id: Uuid::nil(),
        name: data.0.name,
        email: data.0.email,
        username: data.0.username,
        is_licensed: false,
        license_id: empty.clone(),
    };

    info!("/register -> member by API: {:?}", member);

    // Subscribe to Membership Events
    info!("/register -> Subscribe to Membership Events");
    // Connect to server (worker) service
    let mut client = MembershipClient::connect("http://[::1]:50051").await?;
    info!("/register -> connected to server (worker) service @ http://[::1]:50051");
    // # Add new member
    let request = tonic::Request::new(AddMemberRequest {
        name: member.name.clone(),
        email: member.email.clone(),
        username: member.username.clone(),
    });
    let response = client.add_member(request).await?;
    info!("/register -> AddMemberResponse = {:?}", response);

    // Update member status
    let incoming_uuid = response.get_ref().userid.clone();
    member.id = Uuid::parse_str(incoming_uuid.as_str())?;
    info!("/register -> member after AddMember event: {:?}", member);

    // # License a member
    let request = tonic::Request::new(LicenseMemberRequest {
        userid: member.id.clone().to_string(),
        name: member.name.clone(),
    });
    let response = client.license_member(request).await?;
    info!("/register -> LicenseMemberResponse = {:?}", response);

    // Update member status with license information
    let incoming_license_id = response.get_ref().license_id.clone();
    member.is_licensed = true;
    member.license_id = incoming_license_id;
    info!(
        "/register -> member after LicenseMember event: {:?}",
        member
    );

    // Connect to card service
    let mut client = MembershipClient::connect("http://[::1]:50052").await?;
    info!("/register -> connected to card service @ http://[::1]:50052");

    // # Print Card
    let request = tonic::Request::new(PrintCardRequest {
        userid: member.id.clone().to_string(),
        name: member.name.clone(),
    });
    let response = client.print_card(request).await?;
    info!(
        "/register -> PrintResponse of PrintCardRequest = {:?}",
        response
    );

    // Connect to License service
    let mut client = MembershipClient::connect("http://[::1]:50053").await?;
    info!("/register -> connected to card service @ http://[::1]:50053");

    // # Print the certificate
    let request = tonic::Request::new(PrintLicenseRequest {
        userid: member.id.clone().to_string(),
        name: member.name.clone(),
        license_id: member.license_id.clone(),
    });
    let response = client.print_license(request).await?;
    info!(
        "/register -> PrintResponse of PrintLicenseRequest = {:?}",
        response
    );

    // ===

    Ok(HttpResponse::Ok()
        .json("from `/register` API which used for registering new membership api"))
}
