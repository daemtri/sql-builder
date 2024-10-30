pub use crate::error::SqlBuilderError;
pub use crate::name::SqlName;
//pub use crate::where::WhereBuilder;
use anyhow::Result;

/// Main SQL builder
#[derive(Clone)]
pub struct SelectBuilder {
    table: String,
    joins: Vec<String>,
    fields: Vec<String>,
    group_by: Vec<String>,
    having: Option<String>,
    unions: String,
    wheres: Vec<String>,
    order_by: Vec<String>,
    limit: Option<String>,
    offset: Option<String>,
    error: Option<SqlBuilderError>,
}

impl SelectBuilder {
    /// Default constructor for struct
    fn default() -> Self {
        Self {
            table: String::new(),
            joins: Vec::new(),
            fields: Vec::new(),
            group_by: Vec::new(),
            having: None,
            unions: String::new(),
            wheres: Vec::new(),
            order_by: Vec::new(),
            limit: None,
            offset: None,
            error: None::<SqlBuilderError>,
        }
    }

    pub fn select_from<S: ToString>(table: S) -> Self {
        Self {
            table: table.to_string(),
            ..Self::default()
        }
    }

    pub fn and_table<S: ToString>(&mut self, table: S) -> &mut Self {
        self.table = format!("{}, {}", self.table, table.to_string());
        self
    }

    pub fn left_join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = String::from("LEFT JOIN ");
        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    pub fn right_join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = String::from("RIGHT JOIN ");
        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    pub fn inner_join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = String::from("INNER JOIN ");
        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    pub fn cross_join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = String::from("CROSS JOIN ");
        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    pub fn join<S: ToString>(&mut self, table: S) -> &mut Self {
        let mut text = String::from("JOIN ");
        text.push_str(&table.to_string());

        self.joins.push(text);
        self
    }

    pub fn on<S: ToString>(&mut self, constraint: S) -> &mut Self {
        if let Some(last) = self.joins.last_mut() {
            last.push_str(" ON ");
            last.push_str(&constraint.to_string());
        }
        self
    }

    pub fn fields<S, I>(&mut self, fields: I) -> &mut Self
    where
        S: ToString,
        I: IntoIterator<Item = S>,
    {
        let mut fields = fields
            .into_iter()
            .map(|f| f.to_string())
            .collect::<Vec<String>>();
        self.fields.append(&mut fields);
        self
    }

    pub fn field<S: ToString>(&mut self, field: S) -> &mut Self {
        self.fields.push(field.to_string());
        self
    }

    pub fn group_by<S: ToString>(&mut self, field: S) -> &mut Self {
        self.group_by.push(field.to_string());
        self
    }

    pub fn having<S: ToString>(&mut self, cond: S) -> &mut Self {
        self.having = Some(cond.to_string());
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

    pub fn union<S: ToString>(&mut self, query: S) -> &mut Self {
        let append = format!(" UNION {}", &query.to_string());
        self.unions.push_str(&append);
        self
    }

    pub fn union_all<S: ToString>(&mut self, query: S) -> &mut Self {
        self.unions.push_str(" UNION ALL ");
        self.unions.push_str(&query.to_string());
        self
    }

    pub fn order_by<S: ToString>(&mut self, field: S) -> &mut Self {
        let order = field.to_string();
        self.order_by.push(order);
        self
    }

    pub fn limit<S: ToString>(&mut self, limit: S) -> &mut Self {
        self.limit = Some(limit.to_string());
        self
    }

    pub fn offset<S: ToString>(&mut self, offset: S) -> &mut Self {
        self.offset = Some(offset.to_string());
        self
    }

    pub fn build(&self) -> Result<String> {
        if let Some(err) = &self.error {
            return Err(err.clone().into());
        }
        if self.table.is_empty() {
            return Err(SqlBuilderError::NoTableName.into());
        }

        // Build query
        let mut text = self.query()?;
        text.push(';');
        Ok(text)
    }

    pub fn query(&self) -> Result<String> {
        if let Some(err) = &self.error {
            return Err(err.clone().into());
        }

        // Make fields
        let fields = if self.fields.is_empty() {
            "*".to_string()
        } else {
            self.fields.join(", ")
        };

        // Make JOIN parts
        let joins = if self.joins.is_empty() {
            String::new()
        } else {
            format!(" {}", self.joins.join(" "))
        };

        // Make GROUP BY part
        let group_by = if self.group_by.is_empty() {
            String::new()
        } else {
            let having = if let Some(having) = &self.having {
                format!(" HAVING {}", having)
            } else {
                String::new()
            };
            format!(" GROUP BY {}{}", self.group_by.join(", "), having)
        };

        // Make WHERE part
        let wheres = Self::make_wheres(&self.wheres);

        // Make ORDER BY part
        let order_by = if self.order_by.is_empty() || !self.unions.is_empty() {
            String::new()
        } else {
            format!(" ORDER BY {}", self.order_by.join(", "))
        };

        // Make LIMIT part
        let limit = match &self.limit {
            Some(limit) => format!(" LIMIT {}", limit),
            None => String::new(),
        };

        // Make OFFSET part
        let offset = match &self.offset {
            Some(offset) => format!(" OFFSET {}", offset),
            None => String::new(),
        };

        // Make SQL
        let sql = format!("SELECT {fields} FROM {table}{joins}{wheres}{group_by}{unions}{order_by}{limit}{offset}",
                          fields = fields,
                          table = &self.table,
                          joins = joins,
                          group_by = group_by,
                          wheres = wheres,
                          unions = &self.unions,
                          order_by = order_by,
                          limit = limit,
                          offset = offset,
        );
        Ok(sql)
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
    /// Set error during build.
    fn set_error(&mut self, err: &SqlBuilderError) -> &mut Self {
        self.error = Some(err.clone());
        self
    }
}
