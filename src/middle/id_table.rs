use crate::front::ast::Decl;
use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
pub struct IdentificationTable<'i> {
    pub level: isize,
    pub id_table: HashMap<isize, HashMap<String, &'i Decl>>,
}

impl<'i> IdentificationTable<'i> {
    pub fn new() -> Self {
        let mut id_table = HashMap::new();
        id_table.insert(0isize, HashMap::new());

        IdentificationTable { level: 0, id_table }
    }

    pub fn save_attr(&mut self, id: &str, attr: &'i Decl) {
        self.id_table
            .get_mut(&self.level)
            .unwrap()
            .insert(id.to_owned(), attr);
    }

    pub fn get_attr(&self, search_id: &str) -> Option<&Decl> {
        let mut level = self.level;

        while level >= 0 {
            for (id, attr) in self.id_table.get(&level).unwrap().iter() {
                if id == search_id {
                    return Some(attr);
                }
            }

            level -= 1;
        }

        None
    }

    pub fn open_scope(&mut self) {
        self.level += 1;
        self.id_table.insert(self.level, HashMap::new());
    }

    pub fn close_scope(&mut self) {
        self.id_table.remove(&self.level);
        self.level -= 1;
    }
}

impl fmt::Display for IdentificationTable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut level = self.level;

        while level >= 0 {
            let _ = write!(f, "Entries for level {}", level);

            if let Some(mapping) = self.id_table.get(&level) {
                for (id, attr) in mapping.iter() {
                    let _ = write!(f, "\t{:?} => {:?}\n", id, attr);
                }
            }
            level -= 1;
        }
        write!(f, "\n")
    }
}
