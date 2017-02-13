use std::fmt::Debug;

#[derive(Debug, Clone, Copy)]
pub enum ObjectType {
    Integer64,
    Boolean,
}

pub trait Object: Debug {
    fn get_type(&self) -> ObjectType;
    fn inspect(&self) -> String;
}

#[derive(Debug, Clone, Copy)]
pub struct Integer {
    pub value: i64
}

impl Object for Integer {
    fn get_type(&self) -> ObjectType {
        ObjectType::Integer64
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Boolean {
    pub value: bool
}

impl Object for Boolean {
    fn get_type(&self) -> ObjectType {
        ObjectType::Boolean
    }

    fn inspect(&self) -> String {
        self.value.to_string()
    }
}

