pub trait Command {
    fn get_command(&self) -> &'static str;
    fn process_command(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>>;
}
