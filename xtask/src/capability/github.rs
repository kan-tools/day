pub trait Github {
    fn read(&self, resource: &str) -> Result<String, String>;
}
