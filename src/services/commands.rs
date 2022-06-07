pub enum CommandList {

}

pub trait Commands {
    fn is_valid_command(&self) -> bool;
    fn execute(&self) -> Result<Ok(), Err()>;
    fn response(&self) -> String;
}