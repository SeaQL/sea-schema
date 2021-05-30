pub struct MySqlPool;

pub mod mysql {
    pub struct MySqlRow;
}

pub mod postgres {
    pub struct PgRow;
}

pub trait Row {}
