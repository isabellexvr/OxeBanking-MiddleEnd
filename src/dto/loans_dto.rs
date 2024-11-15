use serde::{Serialize, Deserialize};


/* ## Loans Request


[POST] - /loans/request

Body:

{
    "customerId": 123,
    "requestedValue": 10000.00,
    "termInMonths": 24
}

Response:

{
    "id": 1,
    "customerId": 123,
    "requestedValue": 10000.00,
    "termInMonths": 24,
    "status": "pending",
    "approvedAt": null,
    "createdAt": "2024-11-06T00:00:00Z",
    "updatedAt": "2024-11-06T00:00:00Z"
}


[GET] - /loans/request?customerId=123

Response:

[
    {  
        "id": 1,
        "customerId": 123,
        "requestedValue": 10000.00,
        "termInMonths": 24,
        "status": "rejected",
        "approvedAt": null,
        "createdAt": "2024-11-06T00:00:00Z",
        "updatedAt": "2024-11-06T00:00:00Z"
    },
    {  
        "id": 2,
        "customerId": 123,
        "requestedValue": 10000.00,
        "termInMonths": 24,
        "status": "approved",
        "approvedAt": "2024-11-06T00:00:00Z",
        "createdAt": "2024-11-06T00:00:00Z",
        "updatedAt": "2024-11-06T00:00:00Z"
    }
]


[GET] - /loans/request/:id

Response:

{
    "id": 2,
    "customerId": 123,
    "requestedValue": 10000.00,
    "termInMonths": 24,
    "status": "approved",
    "approvedAt": "2024-11-06T00:00:00Z",
    "createdAt": "2024-11-06T00:00:00Z",
    "updatedAt": "2024-11-06T00:00:00Z"
}


[DELETE] - /loans/request/:id

Response:

{
    "message": "Loan request deleted"
}



## Loans


[GET] - /loans?customerId=123

Response:

[
    {
        "id": 1,
        "requestId": 2,
        "customerId": 123,
        "approvedValue": 10000.00,
        "interestRate": 14.0,
        "installmentValue": 2007.46,
        "createdAt": "2024-11-06T00:00:00Z",
        "updatedAt": "2024-11-06T00:00:00Z"
    }
]


[GET] - /loans/:id

Response:

{
    "id": 1,
    "requestId": 2,
    "customerId": 123,
    "approvedValue": 10000.00,
    "interestRate": 14.0,
    "installmentValue": 2007.46,
    "createdAt": "2024-11-06T00:00:00Z",
    "updatedAt": "2024-11-06T00:00:00Z"
}



## Loans Payments


[GET] - /loans/:id/payments

Response:

[
    {
        "id": 1,
        "loanId": 1,
        "paidValue": 2007.46,
        "latePenalty": 0.00,
        "paymentDate": null,
        "status": false,
        "createdAt": "2024-11-05T00:00:00Z",
        "updatedAt": "2024-11-05T00:00:00Z",
        "dueDate": "2024-12-05T00:00:00Z"
    },
    {
        "id": 2,
        "loanId": 1,
        "paidValue": 2007.46,
        "latePenalty": 0.00,
        "paymentDate": null,
        "status": false,
        "createdAt": "2024-11-05T00:00:00Z",
        "updatedAt": "2024-11-05T00:00:00Z",
        "dueDate": "2025-01-05T00:00:00Z"
    },
    {
        "id": 3,
        "loanId": 1,
        "paidValue": 2007.46,
        "latePenalty": 0.00,
        "paymentDate": null,
        "status": false,
        "createdAt": "2024-11-05T00:00:00Z",
        "updatedAt": "2024-11-05T00:00:00Z",
        "dueDate": "2025-02-05T00:00:00Z"
    },
    ...
    {
        "id": 24,
        "loanId": 1,
        "paidValue": 2007.46,
        "latePenalty": 0.00,
        "paymentDate": null,
        "status": false,
        "createdAt": "2024-11-05T00:00:00Z",
        "updatedAt": "2024-11-05T00:00:00Z",
        "dueDate": "2026-11-05T00:00:00Z"
    }
]


[POST] - /loans/:id/payments

Body:

{
    "id": 1,
}

Response:

{
    "id": 1,
    "loanId": 1,
    "paidValue": 2007.46,
    "latePenalty": 0.00,
    "paymentDate": "2024-11-05T00:00:00Z",
    "status": true,
    "createdAt": "2024-11-05T00:00:00Z",
    "updatedAt": "2024-11-05T00:00:00Z",
    "dueDate": "2024-12-05T00:00:00Z"
}, */

#[derive(Debug, Serialize, Deserialize)]
pub struct LoansRequestDTO {
    pub customerId: i32,
    pub requestedValue: f64,
    pub termInMonths: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoansRequestResponseDTO {
    pub id: i32,
    pub customerId: i32,
    pub requestedValue: f64,
    pub termInMonths: i32,
    pub status: String,
    pub approvedAt: Option<String>,
    pub createdAt: String,
    pub updatedAt: String,
}