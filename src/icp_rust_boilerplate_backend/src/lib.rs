#[macro_use]
extern crate serde;
use candid::{Decode, Encode, Principal};
use ic_cdk::api::caller;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use regex::Regex;
use std::{borrow::Cow, cell::RefCell};

// Memory Management
type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

// Import the models module
mod models;
use models::*;

// Define an Error enum for handling errors
#[derive(candid::CandidType, Clone, Serialize, Deserialize)]
pub enum Error {
    Success { msg: String },
    Error { msg: String },
    NotFound { msg: String },
    InvalidPayload { msg: String },
    Unauthorized { msg: String },
    PaymentFailed { msg: String },
    PaymentCompleted { msg: String },
}

// Thread-local storage
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter")
    );

    static USERS_STORAGE: RefCell<StableBTreeMap<u64, User, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))))
    );

    static MOTORCYCLES_STORAGE: RefCell<StableBTreeMap<u64, Motorcycle, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(2))))
    );

    static LOANS_STORAGE: RefCell<StableBTreeMap<u64, Loan, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(3))))
    );

    static PAYMENTS_STORAGE: RefCell<StableBTreeMap<u64, Payment, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(4))))
    );

    static INVESTORS_STORAGE: RefCell<StableBTreeMap<u64, Investor, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(5))))
    );

    static LOAN_POOLS_STORAGE: RefCell<StableBTreeMap<u64, LoanPool, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(6))))
    );
}

// Implement Storable for User
impl Storable for User {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for User {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Motorcycle
impl Storable for Motorcycle {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Motorcycle {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Loan
impl Storable for Loan {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Loan {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Payment
impl Storable for Payment {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Payment {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for Investor
impl Storable for Investor {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Investor {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Implement Storable for LoanPool
impl Storable for LoanPool {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for LoanPool {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// Helper Functions

// Generates a unique identifier for objects
fn generate_uuid() -> u64 {
    let id = ID_COUNTER.with(|counter| {
        let current_id = *counter.borrow().get();
        let _ = counter.borrow_mut().set(current_id + 1);
        current_id
    });

    id
}

// Validate email format
fn validate_email_format(email: &str) -> Result<(), String> {
    let email_regex = Regex::new(r"^\S+@\S+\.\S+$").unwrap();
    if !email_regex.is_match(email) {
        Err("Invalid email format".to_string())
    } else {
        Ok(())
    }
}

// Validate email uniqueness
fn validate_email_uniqueness(email: &str) -> Result<(), String> {
    let email_exists =
        USERS_STORAGE.with(|storage| storage.borrow().iter().any(|(_, user)| user.email == email));

    if email_exists {
        Err("User with this email already exists".to_string())
    } else {
        Ok(())
    }
}

// User Functions
#[ic_cdk::update]
fn register_user(payload: RegisterUserPayload) -> Result<User, String> {
    if payload.name.is_empty() || payload.email.is_empty() || payload.address.is_empty() {
        return Err("Name, email, and address are required fields".to_string());
    }
    validate_email_format(&payload.email)?;
    validate_email_uniqueness(&payload.email)?;

    let id = generate_uuid();
    let user = User {
        id,
        owner: caller(),
        name: payload.name,
        email: payload.email,
        address: payload.address,
        role: payload.role,
    };

    USERS_STORAGE.with(|users| {
        users.borrow_mut().insert(id, user.clone());
        Ok(user)
    })
}

#[ic_cdk::update]
fn update_user(payload: UpdateUserPayload) -> Result<User, String> {
    if !USERS_STORAGE.with(|storage| storage.borrow().contains_key(&payload.id)) {
        return Err("User not found".to_string());
    }
    validate_email_format(&payload.email)?;
    let email_exists = USERS_STORAGE.with(|storage| {
        storage
            .borrow()
            .iter()
            .any(|(_, user)| user.email == payload.email && user.id != payload.id)
    });

    if email_exists {
        return Err("User with this email already exists".to_string());
    }

    let user = User {
        id: payload.id,
        owner: caller(),
        name: payload.name,
        email: payload.email,
        address: payload.address,
        role: payload.role,
    };

    USERS_STORAGE.with(
        |storage| match storage.borrow_mut().insert(payload.id, user.clone()) {
            Some(_) => Ok(user),
            None => Err("User not found".to_string()),
        },
    )
}

#[ic_cdk::query]
fn get_user(id: u64) -> Result<User, String> {
    USERS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(user) => Ok(user.clone()),
        None => Err(format!("User with ID {} not found", id)),
    })
}

#[ic_cdk::query]
fn get_all_users() -> Result<Vec<User>, String> {
    USERS_STORAGE.with(|storage| {
        let users: Vec<User> = storage
            .borrow()
            .iter()
            .map(|(_, user)| user.clone())
            .collect();
        if users.is_empty() {
            Err("No users found".to_string())
        } else {
            Ok(users)
        }
    })
}

// Motorcycle Functions
#[ic_cdk::update]
fn register_motorcycle(payload: RegisterMotorcyclePayload) -> Result<Motorcycle, String> {
    if payload.model.is_empty() || payload.manufacturer.is_empty() {
        return Err("Model and manufacturer are required fields".to_string());
    }

    let id = generate_uuid();
    let motorcycle = Motorcycle {
        id,
        model: payload.model,
        manufacturer: payload.manufacturer,
        price: payload.price,
        status: MotorcycleStatus::Available,
    };

    MOTORCYCLES_STORAGE.with(|motorcycles| {
        motorcycles.borrow_mut().insert(id, motorcycle.clone());
        Ok(motorcycle)
    })
}

#[ic_cdk::query]
fn get_motorcycle(id: u64) -> Result<Motorcycle, String> {
    MOTORCYCLES_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(motorcycle) => Ok(motorcycle.clone()),
        None => Err(format!("Motorcycle with ID {} not found", id)),
    })
}

#[ic_cdk::query]
fn get_all_motorcycles() -> Result<Vec<Motorcycle>, String> {
    MOTORCYCLES_STORAGE.with(|storage| {
        let motorcycles: Vec<Motorcycle> = storage
            .borrow()
            .iter()
            .map(|(_, motorcycle)| motorcycle.clone())
            .collect();
        if motorcycles.is_empty() {
            Err("No motorcycles found".to_string())
        } else {
            Ok(motorcycles)
        }
    })
}

#[ic_cdk::update]
fn update_motorcycle_status(id: u64, status: MotorcycleStatus) -> Result<Motorcycle, String> {
    MOTORCYCLES_STORAGE.with(|storage| {
        let mut motorcycles = storage.borrow_mut();
        if let Some(mut motorcycle) = motorcycles.get(&id) {
            motorcycle.status = status;
            motorcycles.insert(id, motorcycle.clone());
            Ok(motorcycle)
        } else {
            Err(format!("Motorcycle with ID {} not found", id))
        }
    })
}

// Loan Functions
#[ic_cdk::update]
fn create_loan(payload: ApplyLoanPayload) -> Result<Loan, String> {

    let id = generate_uuid();
    let loan = Loan {
        id,
        borrower_id: payload.borrower_id,
        motorcycle_id: payload.motorcycle_id,
        principal_amount: payload.principal_amount,
        interest_rate: payload.interest_rate,
        daily_payment: payload.daily_payment,
        start_date: payload.start_date,
        end_date: payload.end_date,
        status: LoanStatus::Active,
        total_paid: 0.0,
    };

    LOANS_STORAGE.with(|loans| {
        loans.borrow_mut().insert(id, loan.clone());
        Ok(loan)
    })
}

#[ic_cdk::query]
fn get_loan(id: u64) -> Result<Loan, String> {
    LOANS_STORAGE.with(|storage| match storage.borrow().get(&id) {
        Some(loan) => Ok(loan.clone()),
        None => Err(format!("Loan with ID {} not found", id)),
    })
}

#[ic_cdk::query]
fn get_all_loans() -> Result<Vec<Loan>, String> {
    LOANS_STORAGE.with(|storage| {
        let loans: Vec<Loan> = storage
            .borrow()
            .iter()
            .map(|(_, loan)| loan.clone())
            .collect();
        if loans.is_empty() {
            Err("No loans found".to_string())
        } else {
            Ok(loans)
        }
    })
}

#[ic_cdk::update]
fn update_loan_status(id: u64, status: LoanStatus) -> Result<Loan, String> {
    LOANS_STORAGE.with(|storage| {
        let mut loans = storage.borrow_mut();
        if let Some(mut loan) = loans.get(&id) {
            loan.status = status;
            loans.insert(id, loan.clone());
            Ok(loan)
        } else {
            Err(format!("Loan with ID {} not found", id))
        }
    })
}

// Payment Functions
#[ic_cdk::update]
fn create_payment(payload: MakePaymentPayload) -> Result<Payment, String> {
    if payload.amount <= 0.0 {
        return Err("Invalid payment amount".to_string());
    }

    let id = generate_uuid();
    let payment = Payment {
        id,
        loan_id: payload.loan_id,
        borrower_id: payload.borrower_id,
        amount: payload.amount,
        status: PaymentStatus::Pending, // Corrected enum variant
        date: ic_cdk::api::time().to_string(), // Converted timestamp to String
    };

    PAYMENTS_STORAGE.with(|payments| {
        payments.borrow_mut().insert(id, payment.clone());
        Ok(payment)
    })
}

#[ic_cdk::query]
fn get_all_payments_for_loan(loan_id: u64) -> Result<Vec<Payment>, String> {
    PAYMENTS_STORAGE.with(|storage| {
        let payments: Vec<Payment> = storage
            .borrow()
            .iter()
            .filter(|(_, payment)| payment.loan_id == loan_id)
            .map(|(_, payment)| payment.clone())
            .collect();
        if payments.is_empty() {
            Err("No payments found for this loan".to_string())
        } else {
            Ok(payments)
        }
    })
}

// Investor and Loan Pool Functions
#[ic_cdk::update]
fn register_investor(payload: RegisterInvestorPayload) -> Result<Investor, String> {
    if payload.name.is_empty() || payload.email.is_empty() {
        return Err("Name and email are required fields".to_string());
    }

    let email_exists = INVESTORS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, inv)| inv.email == payload.email)
    });

    if email_exists {
        return Err("An investor with this email already exists".to_string());
    }

    let id = generate_uuid();
    let investor = Investor {
        id,
        owner: caller(),
        name: payload.name,
        email: payload.email,
        total_invested: 0.0,
        active_loans: vec![],
        returns_earned: 0.0,
    };

    INVESTORS_STORAGE.with(|investors| {
        investors.borrow_mut().insert(id, investor.clone());
        Ok(investor)
    })
}


#[ic_cdk::update]
fn create_loan_pool(payload: CreateLoanPoolPayload) -> Result<LoanPool, String> {
    if payload.initial_funds <= 0.0 {
        return Err("Invalid pool amount".to_string());
    }

    let name_exists = LOAN_POOLS_STORAGE.with(|storage| {
        storage.borrow().iter().any(|(_, pool)| pool.name == payload.name)
    });

    if name_exists {
        return Err("A loan pool with this name already exists".to_string());
    }

    let id = generate_uuid();
    let pool = LoanPool {
        id,
        name: payload.name,
        total_funds: payload.initial_funds,
        available_funds: payload.initial_funds,
        investor_ids: vec![],
        active_loans: vec![],
    };

    LOAN_POOLS_STORAGE.with(|pools| {
        pools.borrow_mut().insert(id, pool.clone());
        Ok(pool)
    })
}


#[ic_cdk::query]
fn get_all_loan_pools() -> Result<Vec<LoanPool>, String> {
    LOAN_POOLS_STORAGE.with(|storage| {
        let pools: Vec<LoanPool> = storage
            .borrow()
            .iter()
            .map(|(_, pool)| pool.clone())
            .collect();
        if pools.is_empty() {
            Err("No loan pools found".to_string())
        } else {
            Ok(pools)
        }
    })
}

// Function to allocate funds from a loan pool to a loan
#[ic_cdk::update]
fn allocate_funds_from_pool(pool_id: u64, loan_id: u64, amount: f64) -> Result<LoanPool, String> {
    LOAN_POOLS_STORAGE.with(|pools| {
        let mut pools = pools.borrow_mut();
        if let Some(mut pool) = pools.get(&pool_id) {
            if pool.available_funds < amount {
                return Err("Insufficient funds in the pool".to_string());
            }

            // Update the loan
            let loan_result = LOANS_STORAGE.with(|loans| {
                let mut loans = loans.borrow_mut();
                if let Some(mut loan) = loans.get(&loan_id) {
                    loan.principal_amount += amount;
                    loans.insert(loan_id, loan);
                    Ok(())
                } else {
                    Err(format!("Loan with ID {} not found", loan_id))
                }
            });

            // Check if the loan update succeeded
            if let Err(err) = loan_result {
                return Err(err);
            }

            // Update the pool's available funds
            pool.available_funds -= amount;
            pool.active_loans.push(loan_id);
            pools.insert(pool_id, pool.clone());
            Ok(pool)
        } else {
            Err(format!("Loan pool with ID {} not found", pool_id))
        }
    })
}

#[ic_cdk::query]
fn calculate_loan_balance(loan_id: u64) -> Result<f64, String> {
    LOANS_STORAGE.with(|loans| {
        let loans = loans.borrow();
        if let Some(loan) = loans.get(&loan_id) {
            let balance = loan.principal_amount - loan.total_paid;
            Ok(balance)
        } else {
            Err(format!("Loan with ID {} not found", loan_id))
        }
    })
}


#[ic_cdk::update]
fn make_payment(payload: MakePaymentPayload) -> Result<Payment, String> {
    if payload.amount <= 0.0 {
        return Err("Invalid payment amount".to_string());
    }

    let payment_id = generate_uuid();
    let payment = Payment {
        id: payment_id,
        loan_id: payload.loan_id,
        borrower_id: payload.borrower_id,
        amount: payload.amount,
        status: PaymentStatus::Completed,
        date: ic_cdk::api::time().to_string(),
    };

    // Update the loan's total paid
    let loan_update_result = LOANS_STORAGE.with(|loans| {
        let mut loans = loans.borrow_mut();
        if let Some(mut loan) = loans.get(&payload.loan_id) {
            loan.total_paid += payload.amount;
            loans.insert(payload.loan_id, loan);
            Ok(())
        } else {
            Err(format!("Loan with ID {} not found", payload.loan_id))
        }
    });

    // Return an error if the loan update failed
    if let Err(err) = loan_update_result {
        return Err(err);
    }

    // Store the payment record
    let payment_store_result = PAYMENTS_STORAGE.with(|payments| {
        payments.borrow_mut().insert(payment_id, payment.clone());
        Ok(())
    });

    // Return an error if the payment store failed
    if let Err(err) = payment_store_result {
        return Err(err);
    }

    // Return the created payment
    Ok(payment)
}


// Exporting the Candid interface
ic_cdk::export_candid!();
