#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Goal,
    Total,
    Creator,
    Donation(Address),
}

#[contract]
pub struct Crowdfunding;

#[contractimpl]
impl Crowdfunding {

    pub fn initialize(env: Env, goal: i128, creator: Address) {
        creator.require_auth();

        env.storage().instance().set(&DataKey::Goal, &goal);
        env.storage().instance().set(&DataKey::Total, &0i128);
        env.storage().instance().set(&DataKey::Creator, &creator);
    }

    pub fn donate(env: Env, donor: Address, amount: i128) {
        donor.require_auth();

        let mut total: i128 =
            env.storage().instance().get(&DataKey::Total).unwrap_or(0);

        total += amount;
        env.storage().instance().set(&DataKey::Total, &total);

        let key = DataKey::Donation(donor.clone());
        let prev: i128 =
            env.storage().instance().get(&key).unwrap_or(0);

        env.storage().instance().set(&key, &(prev + amount));

        // 🔥 EVENT
        env.events().publish(("donated", donor), amount);
    }

    pub fn refund(env: Env, donor: Address) {
        donor.require_auth();

        let goal: i128 =
            env.storage().instance().get(&DataKey::Goal).unwrap();

        let mut total: i128 =
            env.storage().instance().get(&DataKey::Total).unwrap();

        if total >= goal {
            panic!("Goal reached, refund not allowed");
        }

        let key = DataKey::Donation(donor.clone());
        let donated: i128 =
            env.storage().instance().get(&key).unwrap_or(0);

        if donated == 0 {
            panic!("No donation found");
        }

        total -= donated;
        env.storage().instance().set(&DataKey::Total, &total);
        env.storage().instance().set(&key, &0i128);

        // 🔥 EVENT
        env.events().publish(("refunded", donor), donated);
    }

    pub fn withdraw(env: Env) {
        let creator: Address =
            env.storage().instance().get(&DataKey::Creator).unwrap();

        creator.require_auth();

        let goal: i128 =
            env.storage().instance().get(&DataKey::Goal).unwrap();

        let total: i128 =
            env.storage().instance().get(&DataKey::Total).unwrap();

        if total < goal {
            panic!("Goal not reached");
        }

        env.storage().instance().set(&DataKey::Total, &0i128);

        // 🔥 EVENT
        env.events().publish(("withdrawn", creator), total);
    }

    pub fn get_total(env: Env) -> i128 {
        env.storage().instance().get(&DataKey::Total).unwrap_or(0)
    }
}