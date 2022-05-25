// use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Member {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub username: String,
    pub is_licensed: bool,
    pub license_id: String,
}

// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemberRequest {
    pub name: String,
    pub email: String,
    pub username: String,
}

// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LicenseMemberRequest {
    pub user_id: Uuid,
    pub name: String,
}

// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintCardRequest {
    pub user_id: Uuid,
    pub name: String,
}

// ===

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrintLicenseRequest {
    pub user_id: Uuid,
    pub name: String,
    pub license_id: String,
}
