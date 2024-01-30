use crate::schema::extensions;
use diesel::{Insertable, Queryable, Selectable, AsChangeset};

#[derive(Insertable)]
#[diesel(table_name = extensions)]
pub struct NewExtension<'a> {
    pub mac: &'a str,
    pub extension: i32,
    pub domain: &'a str,
    pub timer: &'a str,
    pub mail: bool,
}

#[derive(Debug, Queryable, Selectable, AsChangeset)]
#[diesel(table_name = extensions)]
pub struct Extension {
    pub mac: String,
    pub extension: i32,
    pub domain: String,
    pub timer: String,
    pub mail: bool,
}