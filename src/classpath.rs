use crate::entry::Entry;
use crate::entry::new_entry;
use std::string::ToString;

pub struct Classpath<'a> {
    // boot_classpath,
    // ext_classpath,
    user_classpath: Box<dyn Entry + 'a>
}

impl<'a> Classpath<'a> {
    pub fn parse(cp_option: String) -> Classpath<'a> {
        let entry: Box<dyn Entry + 'a> = new_entry(cp_option);
        Self { user_classpath: entry }
    }
    pub fn read_class(&self, class_name:String) ->  Option<(Vec<u8>, &dyn Entry)> {
        self.user_classpath.read_class(class_name)
    }
}

impl ToString for Classpath<'_> {
    fn to_string(&self) -> String {
        self.user_classpath.to_string()
    }
}