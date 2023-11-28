
pub trait TableDataTraits<T> {
    fn load_table_data(&self) -> Vec<T>;
    fn get_table_headers(&self) -> Vec<String>;
    fn get_line(&self, index: usize) -> T;
    fn get_line_as_string(&self, index: usize) -> String;
    fn get_lines(&self, first_line: usize, last_line: usize) -> Vec<T>;
}

