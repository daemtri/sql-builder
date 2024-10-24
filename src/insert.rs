use crate::arg::SqlArg;

pub struct Insert {
    pub(crate) table_name: String,
    pub(crate) fields: String,
    pub(crate) values: String,
}

impl Insert {
    pub fn new() -> Self {
        Self {
            table_name: String::new(),
            fields: String::new(),
            values: String::new(),
        }
    }

    pub fn into<S: ToString>(&mut self, table_name: S) -> &mut Self {
        self.table_name = table_name.to_string();
        self
    }

    pub fn field_value<T: SqlArg>(&mut self, field_name: &str, value: T) -> &mut Self {
        if self.fields.len() > 0 {
            self.fields.push_str(format!(", {}", field_name).as_str());
            self.values
                .push_str(format!(", {}", value.sql_arg()).as_str());
        } else {
            self.fields.push_str(field_name);
            self.values.push_str(value.sql_arg().as_str());
        }

        self
    }

    pub fn build(&self) -> String {
        format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.table_name, self.fields, self.values
        )
    }
}
