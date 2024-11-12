use serde::{Serialize, Deserialize};

/* 

    insurance:
{
  "insurance_id": "1",
  "claim_number": "456.78",
  "claim_date": "2023-10-01",
  "end_date": "2023-10-15",
  "claim_amount": "1000.50",
  "status": "Pending"
}

    claim:
  {
    "id": 2,
    "insurance_id": 1,
    "claim_number": 456.78,
    "claim_date": "2023-10-01T00:00:00Z",
    "end_date": "2023-10-15T00:00:00Z",
    "claim_amount": 1000.5,
    "status": "Pending"
  }

    log:
  {
  "user_id": "1",
  "protocol_number": "456",
  "date": "2023-10-01",
  "type": "INFO"
}

 */

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceGetDTO {
    pub id: i32,
    pub user_id: i32,
    pub max_coverage: f64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InsuranceReceiveDTO {
    pub user_id: i32,
    pub max_coverage: f64,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimGetDTO {
    pub id: i32,
    pub insurance_id: i32,
    pub claim_number: f64,
    pub claim_date: String,
    pub end_date: String,
    pub claim_amount: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ClaimReceiveDTO {
    pub insurance_id: i32,
    pub claim_number: f64,
    pub claim_date: String,
    pub end_date: String,
    pub claim_amount: f64,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogGetDTO {
    pub id: i32,
    pub user_id: i32,
    pub protocol_number: String,
    pub date: String,
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogReceiveDTO {
    pub user_id: i32,
    pub protocol_number: String,
    pub date: String,
    pub type_: String,
}