pub trait BaseCalculate {
    fn calculate(&self) -> i64;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
}