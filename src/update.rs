use crate::arg::SqlArg;
pub use crate::error::SqlBuilderError;
pub use crate::name::SqlName;
//pub use crate::where::WhereBuilder;
use anyhow::Result;

/// Main SQL builder
#[derive(Clone)]
pub struct UpdateBuilder {
    table: String,
    sets: Vec<String>,
    returning: Option<String>,
    wheres: Vec<String>,
    error: Option<SqlBuilderError>,
}

impl UpdateBuilder {
    /// Default constructor for struct
    fn default() -> Self {
        Self {
            table: String::new(),
            sets: Vec::new(),
            returning: None,
            wheres: Vec::new(),
            error: None::<SqlBuilderError>,
        }
    }

    pub fn update_table<S: ToString>(table: S) -> Self {
        Self {
            table: table.to_string(),
            ..Self::default()
        }
    }

    pub fn set<S, T>(&mut self, field: S, value: T) -> &mut Self
    where
        S: ToString,
        T: SqlArg,
    {
        let expr = format!("{} = {}", &field.to_string(), &value.sql_arg());
        self.sets.push(expr);
        self
    }

    pub fn and_where<S: ToString>(&mut self, cond: S) -> &mut Self {
        // Checks
        let cond = cond.to_string();
        if cond.is_empty() {
            return self.set_error(&SqlBuilderError::NoWhereCond);
        }

        // Change
        self.wheres.push(cond);
        self
    }

    pub fn or_where<S: ToString>(&mut self, cond: S) -> &mut Self {
        // Checks
        let cond = cond.to_string();
        if cond.is_empty() {
            return self.set_error(&SqlBuilderError::NoWhereCond);
        }

        // Change
        if self.wheres.is_empty() {
            self.wheres.push(cond);
        } else if let Some(last) = self.wheres.last_mut() {
            last.push_str(" OR ");
            last.push_str(&cond);
        }
        self
    }

    /// Set error during build.
    fn set_error(&mut self, err: &SqlBuilderError) -> &mut Self {
        self.error = Some(err.clone());
        self
    }

    pub fn build(&self) -> Result<String> {
        if let Some(err) = &self.error {
            return Err(err.clone().into());
        }
        // Checks
        if let Some(err) = &self.error {
            return Err(err.clone().into());
        }
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }
        if self.sets.is_empty() {
            return Err(SqlBuilderError::NoSetFields.into());
        }

        // Make SET part
        let sets = self.sets.join(", ");

        // Make WHERE part
        let wheres = Self::make_wheres(&self.wheres);

        // Make RETURNING part
        let returning = if let Some(ret) = &self.returning {
            format!(" RETURNING {}", ret)
        } else {
            "".to_string()
        };

        // Make SQL
        let sql = format!(
            "UPDATE {table} SET {sets}{wheres}{returning};",
            table = &self.table,
            sets = sets,
            wheres = wheres,
            returning = returning,
        );
        Ok(sql)
    }

    pub fn returning<S: ToString>(&mut self, field: S) -> &mut Self {
        self.returning = Some(field.to_string());
        self
    }

    /// Make WHERE part
    fn make_wheres(wheres: &[String]) -> String {
        match wheres.len() {
            0 => String::new(),
            1 => {
                let wheres = wheres[0].to_string();
                format!(" WHERE {}", wheres)
            }
            _ => {
                let wheres: Vec<String> = wheres.iter().map(|w| format!("({})", w)).collect();
                format!(" WHERE {}", wheres.join(" AND "))
            }
        }
    }
}
