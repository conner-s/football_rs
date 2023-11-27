
pub trait TableMemberTraits {
    fn get_attribute(&self, index: u16) -> String;
    fn get_attributes(&self) -> Vec<String>;
    fn get_attribute_name(&self, index: u16) -> String;
    fn get_attribute_names(&self) -> Vec<String>;
}

