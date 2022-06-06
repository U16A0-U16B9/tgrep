pub mod reputations;

pub trait Data{
    fn to_string(&self) -> Option<String>;
}