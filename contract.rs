use soroban_sdk::{contractimpl, Address, Env, Symbol, contracttype, Vec};

pub struct Microfinance;

#[contracttype]
pub struct GroupLoan {
    pub group: Vec<Address>,
    pub amount: i128,
    pub repaid: bool,
}

#[contractimpl]
impl Microfinance {
    fn loans<'a>(env: &'a Env) -> Vec<'a, GroupLoan> {
        env.storage().instance().get::<Vec<GroupLoan>>(Symbol::short("loans")).unwrap_or(Vec::new(&env))
    }

    pub fn create_group_loan(env: Env, group: Vec<Address>, amount: i128) {
        let mut loans = Self::loans(&env);
        loans.push_back(GroupLoan { group, amount, repaid: false });
        env.storage().instance().set(Symbol::short("loans"), &loans);
    }

    pub fn repay(env: Env, index: u32) {
        let mut loans = Self::loans(&env);
        loans[index as usize].repaid = true;
        env.storage().instance().set(Symbol::short("loans"), &loans);
    }
}
