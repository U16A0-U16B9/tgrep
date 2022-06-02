pub mod triggers;

pub trait Config{
    fn to_string(&self) -> Option<String>;
}