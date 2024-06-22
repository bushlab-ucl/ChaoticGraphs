pub trait Node {
    fn get_id(&self) -> usize;
    fn get_neighbors(&self) -> Vec<usize>;
    fn get_name(&self) -> &str;
    fn get_info(&self) -> String;
    fn set_info(&mut self, info: String);
}
