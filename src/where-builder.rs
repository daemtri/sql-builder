use std::fmt;

use crate::error::SqlBuilderError;

#[macro_export]
macro_rules! and {
    ( $f:expr, $( $l:expr ),* ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $f .to_string() );
            $(
                x.push_str(") AND (");
                x.push_str( & $l .to_string() );
            )*
            x.push(')');
            x
        }
    };
}

#[macro_export]
macro_rules! or {
    ( $f:expr, $( $l:expr ),* ) => {
        {
            let mut x = String::from( $f );
            $(
                x.push_str(" OR ");
                x.push_str( & $l .to_string() );
            )*
            x
        }
    };
}

#[macro_export]
macro_rules! not {
    ( $f:expr ) => {{
        let mut x = String::from("NOT ");
        x.push_str(&$f.to_string());
        x
    }};
}

#[macro_export]
macro_rules! brackets {
    ( $el:expr ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $el .to_string() );
            x.push(')');
            x
        }
    };
    ( $first:expr, $( $el:expr ),* ) => {
        {
            let mut x = String::from("(");
            x.push_str( & $first .to_string() );
            $(
                x.push_str(", ");
                x.push_str( & $el .to_string() );
            )*
            x.push(')');
            x
        }
    };
}

/// Build WHERE for SQL.
#[derive(Clone, Default)]
pub struct Where {
    text: String,
    prefix: Option<String>,
    error: Option<SqlBuilderError>,
    was_and: bool,
}

impl fmt::Display for Where {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.text)
    }
}

impl Where {
    pub fn new<S>(smth: S) -> Self
    where
        S: ToString,
    {
        // Checks
        let text = smth.to_string();
        if text.is_empty() {
            return Self {
                error: Some(SqlBuilderError::NoWhereField),
                ..Self::default()
            };
        }

        // Create
        Self {
            text,
            ..Self::default()
        }
    }

    pub fn empty() -> Self {
        Self::default()
    }

    pub fn in_brackets(&mut self) -> &mut Self {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }

        // Change
        self.text.insert(0, '(');
        self.text.push(')');
        self
    }

    pub fn not(&mut self) -> &mut Self {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }

        // Change
        self.text.insert_str(0, "NOT ");
        self
    }

    pub fn and<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if !self.was_and {
            self.text.insert(0, '(');
            self.text.push(')');
            self.was_and = true;
        }
        self.text.push_str(" AND (");
        self.text.push_str(&smth);
        self.text.push(')');
        self
    }

    pub fn or<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        if self.text.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereField);
            return self;
        }
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        self.text.push_str(" OR ");
        self.text.push_str(&smth);
        self
    }

    pub fn eq<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" = ");
        self.text.push_str(&smth);
        self
    }

    pub fn ne<S>(&mut self, smth: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let smth = smth.to_string();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" <> ");
        self.text.push_str(&smth);
        self
    }

    pub fn is_in<S>(&mut self, smth: Vec<S>) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let smth = smth.iter().map(|x| x.to_string()).collect::<Vec<String>>();
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        self.text.push_str(" in ");
        self.text.push_str("(");
        self.text.push_str(&smth.join(", "));
        self.text.push_str(")");
        self
    }

    pub fn between<S>(&mut self, min: S, max: S) -> &mut Self
    where
        S: ToString,
    {
        // Checks
        let min = min.to_string();
        let max = max.to_string();

        if min.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        if max.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" BETWEEN ");
        self.text.push_str(&min);
        self.text.push_str(" AND ");
        self.text.push_str(&max);
        self
    }

    pub fn is_null(&mut self) -> &mut Self {
        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" IS NULL");
        self
    }

    pub fn not_null(&mut self) -> &mut Self {
        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" IS NOT NULL");
        self
    }

    pub fn lower_then(&mut self, smth: &str) -> &mut Self {
        // Checks
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" < ");
        self.text.push_str(smth);
        self
    }

    pub fn lower_equal(&mut self, smth: &str) -> &mut Self {
        // Checks
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" <= ");
        self.text.push_str(smth);
        self
    }

    pub fn greater_then(&mut self, smth: &str) -> &mut Self {
        // Checks
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" > ");
        self.text.push_str(smth);
        self
    }

    pub fn greater_equal(&mut self, smth: &str) -> &mut Self {
        // Checks
        if smth.is_empty() {
            self.error = Some(SqlBuilderError::NoWhereValue(self.text.clone()));
            return self;
        }

        // Change
        if let Some(prefix) = &self.prefix {
            self.text.push(' ');
            self.text.push_str(&prefix);
            self.prefix = None;
        }
        self.text.push_str(" >= ");
        self.text.push_str(smth);
        self
    }

    pub fn build(&self) -> Result<String, SqlBuilderError> {
        match &self.error {
            Some(err) => Err(err.clone()),
            None => Ok(self.text.to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_and() {
        let sql = and!("10", "20", "30");
        assert_eq!("(10) AND (20) AND (30)", sql);
    }

    #[test]
    fn test_macro_or() {
        let sql = or!("10", "20", "30");
        assert_eq!("10 OR 20 OR 30", sql);
    }

    #[test]
    fn test_macro_not() {
        let sql = not!("10");
        assert_eq!("NOT 10", sql);
    }

    #[test]
    fn test_macro_brackets() {
        let sql = brackets!("10", "20", "30");
        assert_eq!("(10, 20, 30)", sql);

        let sql = brackets!("10");
        assert_eq!("(10)", sql);
    }

    #[test]
    fn test_macro_and_or_not() {
        let sql = and!("10", or!("20", not!("30"), "40"));
        assert_eq!("(10) AND (20 OR NOT 30 OR 40)", &sql);
    }

    #[test]
    fn test_new_where() {
        let text = Where::new("abc").to_string();
        assert_eq!("abc", &text);
    }

    #[test]
    fn test_where_brackets() {
        let text = Where::new("abc").eq(10).in_brackets().to_string();
        assert_eq!("(abc = 10)", &text);
    }

    #[test]
    fn test_where_build() {
        let res = Where::new("abc").eq(10).build();
        assert_eq!(Ok("abc = 10".to_string()), res);
    }

    #[test]
    fn test_where_not() {
        let text = Where::new("abc").eq(10).in_brackets().not().to_string();
        assert_eq!("NOT (abc = 10)", &text);
    }

    #[test]
    fn test_where_and() {
        let text = Where::new("abc").eq(10).and(20).to_string();
        assert_eq!("(abc = 10) AND (20)", &text);
    }

    #[test]
    fn test_where_or() {
        let text = Where::new("abc").eq(10).or(20).to_string();
        assert_eq!("abc = 10 OR 20", &text);
    }

    #[test]
    fn test_where_eq() {
        let text = Where::new("abc").eq(10).to_string();
        assert_eq!("abc = 10", &text);
    }

    #[test]
    fn test_where_is_in() {
        let text = Where::new("abc").is_in(vec![1, 2, 3]).to_string();
        assert_eq!("abc in (1, 2, 3)", &text);
    }

    #[test]
    fn test_where_ne() {
        let text = Where::new("abc").ne(10).to_string();
        assert_eq!("abc <> 10", &text);
    }

    #[test]
    fn test_where_between() {
        let text = Where::new("abc").between(1, 10).to_string();
        assert_eq!("abc BETWEEN 1 AND 10", &text);
    }

    #[test]
    fn test_where_null() {
        let text = Where::new("abc").is_null().to_string();
        assert_eq!("abc IS NULL", &text);
    }

    #[test]
    fn test_where_not_null() {
        let text = Where::new("abc").not_null().to_string();
        assert_eq!("abc IS NOT NULL", &text);
    }

    #[test]
    fn test_where_lt() {
        let text = Where::new("abc").lower_then("1").to_string();
        assert_eq!("abc < 1", &text);
    }

    #[test]
    fn test_where_le() {
        let text = Where::new("abc").lower_equal("1").to_string();
        assert_eq!("abc <= 1", &text);
    }

    #[test]
    fn test_where_gt() {
        let text = Where::new("abc").greater_then("1").to_string();
        assert_eq!("abc > 1", &text);
    }

    #[test]
    fn test_where_ge() {
        let text = Where::new("abc").greater_equal("1").to_string();
        assert_eq!("abc >= 1", &text);
    }
}
