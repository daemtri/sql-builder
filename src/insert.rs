pub struct Insert {}

impl Insert {
    pub fn new() -> Self {
        Self {}
    }

    pub fn into(self, table_name: &str) -> InsertInto {
        InsertInto::new(table_name)
    }
}
