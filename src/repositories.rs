use diesel::PgConnection;
use crate::schema::*;
use crate::models::*;
use diesel::prelude::*;


pub struct RustaceanRepository;

impl RustaceanRepository {
    pub fn find_multiple(c: &mut &PgConnection, limit: i64) -> QueryResult<Vec<Rustacean>> {
        rustaceans::table.limit(limit).order(rustaceans::id.desc()).load::<Rustacean>(c)
    }

    pub fn find(c: &mut &PgConnection, id: i32) -> QueryResult<Rustacean>{
        rustaceans::table.find(id).get_result::<Rustacean>(c)
    }

    pub fn delete(c: &mut &PgConnection, id: i32)-> QueryResult<usize>{
        diesel::delete(rustaceans::table.find(id)).execute(c)
    }

    pub fn create(c: &mut &PgConnection, new_rustacean: NewRustacean) -> QueryResult<Rustacean> {
        diesel::insert_into(rustaceans::table).values(new_rustacean).get_result(c)
    }

    pub fn save(c: &mut &PgConnection, id: i32, rustaceans: Rustacean){
        diesel::update(rustaceans::table.find(id)).set((rustaceans::email.eq(rustacean.email.to_owned()), rustaceans::name.eq(rustacean.name.to_owned()))).execute(c)?;
        Self::find(c, id)
    }

}

pub struct CrateRepository;

impl CrateRepository {
    pub fn find_multiple(c: &mut &PgConnection, limit: i64) -> QueryResult<Vec<Crates>> {
        crates::table.limit(limit).order(crates::id.desc()).load::<Crates>(c)
    }

    pub fn find(c: &mut &PgConnection, id: i32) -> QueryResult<Crates>{
        crates::table.find(id).get_result::<Crates>(c)
    }

    pub fn delete(c: &mut &PgConnection, id: i32)-> QueryResult<usize>{
        crates::delete(crates::table.find(id)).execute(c)
    }

    pub fn create(c: &mut &PgConnection, new_crates: NewCrates) -> QueryResult<Crates> {
        crates::insert_into(crates::table).values(new_crates).get_result(c)
    }

    pub fn save(c: &mut &PgConnection, id: i32, a_crates: Crates){
        crates::update(crates::table.find(id)).set((
            crates::code.eq(a_crates.code.to_owned()), 
            crates::name.eq(a_crates.name.to_owned()),
            crates::version.eq(a_crates.version.to_owned()), 
            crates::description.eq(a_crates.description.to_owned()), 
            crates::rustacean_id.eq(a_crates.rustacean_id.to_owned()), 
        )).execute(c)?;

        Self::find(c, id)
    }

}