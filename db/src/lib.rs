#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_derive_enum;

#[allow(non_camel_case_types)]
mod types {
#[derive(Debug, DbEnum)]
    pub enum AccountType {
        Org,
        Indirect,
        Beta,
        User,
    }
    pub type Account_type = AccountType;

#[derive(Debug, DbEnum)]
    pub enum BuildCause {
        Manual,
        Push,
        Pr,
    }
    pub type Build_cause = BuildCause;
}

#[allow(unused_imports)]
mod schema;

pub use types::*;
pub use schema::*;
