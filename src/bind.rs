use crate::arg::{SqlArg, SqlArgs};

pub trait Bind {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(100).bind(200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind<S>(&self, arg: S) -> String
    where
        S: SqlArg;

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds((100, "Harry Potter%")))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds<SS>(&self, args: SS) -> String
    where
        SS: SqlArgs;

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name<S>(&self, name: &dyn ToString, arg: S) -> String
    where
        S: SqlArg;
}

impl Bind for &str {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(&100).bind(&200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind<S>(&self, arg: S) -> String
    where
        S: SqlArg,
    {
        (*self).to_string().bind(arg)
    }

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds((100, "Harry Potter%")))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds<SS>(&self, args: SS) -> String
    where
        SS: SqlArgs,
    {
        (*self).to_string().binds(args)
    }

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name<S>(&self, name: &dyn ToString, arg: S) -> String
    where
        S: SqlArg,
    {
        (*self).to_string().bind_name(name, arg)
    }
}

impl Bind for String {
    /// Replace first ? with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price BETWEEN ? AND ?".bind(&100).bind(&200))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price BETWEEN 100 AND 200;", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind<S>(&self, arg: S) -> String
    where
        S: SqlArg,
    {
        self.replacen('?', &arg.sql_arg(), 1)
    }

    /// Cyclic bindings of values.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::select_from("books")
    ///     .fields(&["title", "price"])
    ///     .and_where("price > ? AND title LIKE ?".binds((100, "Harry Potter%")))
    ///     .sql()?;
    ///
    /// assert_eq!("SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn binds<SS>(&self, args: SS) -> String
    where
        SS: SqlArgs,
    {
        let args = args.sql_args();
        let mut offset = 0;
        let mut res = String::new();
        let len = args.len();
        for ch in self.chars() {
            if ch == '?' {
                res.push_str(&args[offset]);
                offset = (offset + 1) % len;
            } else {
                res.push(ch);
            }
        }
        res
    }

    /// Replace all :name: with a value.
    ///
    /// ```
    /// # use std::error::Error;
    /// # use anyhow::Result;
    /// use sql_builder::prelude::*;
    ///
    /// # fn main() -> Result<()> {
    /// let sql = SqlBuilder::insert_into("books")
    ///     .fields(&["title", "price"])
    ///     .values(&[":name:, :costs:"])
    ///     .sql()?
    ///     .bind_name(&"name", &"Harry Potter and the Philosopher's Stone")
    ///     .bind_name(&"costs", &150);
    ///
    /// assert_eq!("INSERT INTO books (title, price) VALUES ('Harry Potter and the Philosopher''s Stone', 150);", &sql);
    /// # Ok(())
    /// # }
    /// ```
    fn bind_name<S>(&self, name: &dyn ToString, arg: S) -> String
    where
        S: SqlArg,
    {
        let rep = format!(":{}:", &name.to_string());
        self.replace(&rep, &arg.sql_arg())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;
    use anyhow::{Ok, Result};

    #[test]
    fn test_bind() -> Result<()> {
        let foo = "f?o?o";

        assert_eq!("'lol'foo?", &"?foo?".bind(&"lol"));
        assert_eq!("'lol'foo10", &"?foo?".bind(&"lol").bind(&10));
        assert_eq!("'lol'foo?", &"?foo?".bind(&String::from("lol")));
        assert_eq!("'lol'foo?", &String::from("?foo?").bind(&"lol"));
        assert_eq!("f'lol'o?o", &foo.bind(&"lol"));
        assert_eq!("fo'f?o?o'o", &"fo?o".bind(&foo));
        assert_eq!("fo10o", &"fo?o".bind(&10_usize));
        assert_eq!("fo10o", &"fo?o".bind(&10));
        assert_eq!("fo10o", &"fo?o".bind(&10_isize));
        assert_eq!("foTRUEo", &"fo?o".bind(&true));
        assert_eq!("foFALSEo", &"fo?o".bind(&false));
        assert_eq!("f'lol'oo:def:", &"f:abc:oo:def:".bind_name(&"abc", &"lol"));

        Ok(())
    }

    #[test]
    fn test_binds() -> Result<()> {
        assert_eq!("10f20o30o10", &"?f?o?o?".binds((10, 20, 30)));
        assert_eq!(
            "'abc'f'def'o'ghi'o'abc'",
            &"?f?o?o?".binds(("abc", "def", "ghi"))
        );
        assert_eq!("10f20o30o10", &String::from("?f?o?o?").binds((10, 20, 30)));
        assert_eq!(
            "10f'AAA'oTRUEo10",
            &String::from("?f?o?o?").binds((10, "AAA", true))
        );
        assert_eq!("1f1.5o0.0000001o1", &"?f?o?o?".binds((1.0, 1.5, 0.0000001)));

        Ok(())
    }

    #[test]
    fn test_bind_doc() -> Result<()> {
        let sql = SqlBuilder::select_from("books")
            .fields(&["title", "price"])
            .and_where("price > ? AND title LIKE ?".binds((100, "Harry Potter%")))
            .sql()?;

        assert_eq!(
            "SELECT title, price FROM books WHERE price > 100 AND title LIKE 'Harry Potter%';",
            &sql
        );

        Ok(())
    }

    #[test]
    fn test_null() -> Result<()> {
        let foo: Option<&str> = None;
        assert_eq!("foNULLo", &"fo?o".bind(&foo));

        let foo = Some("foo");
        assert_eq!("fo'foo'o", &"fo?o".bind(&foo));
        Ok(())
    }

    #[test]
    fn test_in() -> Result<()> {
        assert_eq!("10 IN (1, 2, 3)", &"10 IN (?)".bind(vec![1, 2, 3]));
        Ok(())
    }
}
