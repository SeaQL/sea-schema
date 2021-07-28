pub struct MySqlPool;

pub mod mysql {
    pub struct MySqlRow;
}

pub struct PgPool;

pub mod postgres {
    pub struct PgRow;
}

pub trait Row {}
