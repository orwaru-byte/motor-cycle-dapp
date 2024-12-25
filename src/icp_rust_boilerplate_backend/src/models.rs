use candid::{CandidType, Principal};
use serde::{Deserialize, Serialize};

// User Role Types Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum UserRole {
    #[default]
    Borrower,
    Lender,
    Administrator,
    Investor,
}

// Struct representing a User
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct User {
    pub(crate) id: u64,
    pub(crate) owner: Principal,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Struct representing a Motorcycle
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Motorcycle {
    pub(crate) id: u64,
    pub(crate) model: String,
    pub(crate) manufacturer: String,
    pub(crate) price: f64,
    pub(crate) status: MotorcycleStatus, // Available, In Loan, etc.
}

// Motorcycle Status Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum MotorcycleStatus {
    #[default]
    Available,
    InLoan,
    FullyOwned,
    Repossessed,
}

// Loan Struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Loan {
    pub(crate) id: u64,
    pub(crate) borrower_id: u64,
    pub(crate) motorcycle_id: u64,
    pub(crate) principal_amount: f64,
    pub(crate) interest_rate: f64,
    pub(crate) daily_payment: f64,
    pub(crate) total_paid: f64,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
    pub(crate) status: LoanStatus,
}

// Loan Status Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum LoanStatus {
    #[default]
    Active,
    Completed,
    Defaulted,
}

// Payment Struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub(crate) id: u64,
    pub(crate) loan_id: u64,
    pub(crate) borrower_id: u64,
    pub(crate) amount: f64,
    pub(crate) date: String,
    pub(crate) status: PaymentStatus,
}

// Payment Status Enum
#[derive(CandidType, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub enum PaymentStatus {
    #[default]
    Pending,
    Completed,
    Failed,
}

// Investor Struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Investor {
    pub(crate) id: u64,
    pub(crate) owner: Principal,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) total_invested: f64,
    pub(crate) active_loans: Vec<u64>,
    pub(crate) returns_earned: f64,
}

// Loan Pool Struct
#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct LoanPool {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) total_funds: f64,
    pub(crate) available_funds: f64,
    pub(crate) investor_ids: Vec<u64>,
    pub(crate) active_loans: Vec<u64>,
}

// Payloads

// Register User Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RegisterUserPayload {
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Update User Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct UpdateUserPayload {
    pub(crate) id: u64,
    pub(crate) name: String,
    pub(crate) email: String,
    pub(crate) address: String,
    pub(crate) role: UserRole,
}

// Motorcycle Registration Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RegisterMotorcyclePayload {
    pub(crate) model: String,
    pub(crate) manufacturer: String,
    pub(crate) price: f64,
}

// Loan Application Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct ApplyLoanPayload {
    pub(crate) borrower_id: u64,
    pub(crate) motorcycle_id: u64,
    pub(crate) principal_amount: f64,
    pub(crate) interest_rate: f64,
    pub(crate) daily_payment: f64,
    pub(crate) start_date: String,
    pub(crate) end_date: String,
}

// Make Payment Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct MakePaymentPayload {
    pub(crate) loan_id: u64,
    pub(crate) borrower_id: u64,
    pub(crate) amount: f64,
}

// Investor Registration Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct RegisterInvestorPayload {
    pub(crate) name: String,
    pub(crate) email: String,
}

// Create Loan Pool Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct CreateLoanPoolPayload {
    pub(crate) name: String,
    pub(crate) initial_funds: f64,
}

// Add Funds to Loan Pool Payload
#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct AddFundsPayload {
    pub(crate) pool_id: u64,
    pub(crate) investor_id: u64,
    pub(crate) amount: f64,
}
