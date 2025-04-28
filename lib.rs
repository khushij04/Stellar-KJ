#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, Symbol, String, Vec, symbol_short};

#[contracttype]
#[derive(Clone)]
pub struct Investment {
    pub property_id: String,
    pub amount: u64,
}

#[contracttype]
pub enum InvestmentBook {
    UserInvestment(Address, String), // (User, PropertyID)
}

#[contracttype]
#[derive(Clone)]
pub struct Property {
    pub property_id: String,
    pub funding_goal: u64,
    pub total_raised: u64,
}

#[contracttype]
pub enum PropertyList {
    Property(String),
}

#[contract]
pub struct CrowdfundedRealEstate;

#[contractimpl]
impl CrowdfundedRealEstate {
    pub fn add_property(env: Env, property_id: String, funding_goal: u64) {
        let key = PropertyList::Property(property_id.clone());
        let property = Property {
            property_id: property_id.clone(),
            funding_goal,
            total_raised: 0,
        };
        env.storage().instance().set(&key, &property);
    }

    pub fn invest(env: Env, user: Address, property_id: String, amount: u64) {
        let key = PropertyList::Property(property_id.clone());
        let mut property: Property = env.storage().instance().get(&key).expect("Property not found");

        property.total_raised += amount;
        env.storage().instance().set(&key, &property);

        let invest_key = InvestmentBook::UserInvestment(user.clone(), property_id.clone());
        let mut record = env.storage().instance().get(&invest_key).unwrap_or(Investment {
            property_id: property_id.clone(),
            amount: 0,
        });

        record.amount += amount;
        env.storage().instance().set(&invest_key, &record);
    }

    pub fn view_investment(env: Env, user: Address, property_id: String) -> Investment {
        let key = InvestmentBook::UserInvestment(user, property_id.clone());
        env.storage().instance().get(&key).unwrap_or(Investment {
            property_id,
            amount: 0,
        })
    }

    pub fn view_property(env: Env, property_id: String) -> Property {
        let key = PropertyList::Property(property_id.clone());
        env.storage().instance().get(&key).expect("Property not found")
    }
}
