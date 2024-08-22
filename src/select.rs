use crate::Builder;
use crate::SqlBuilder;
use anyhow::Result;

pub struct Select<T, I>(I)
where
    T: ToString,
    I: IntoIterator<Item = T>;

impl<T, I> Select<T, I>
where
    T: ToString,
    I: IntoIterator<Item = T>,
{
    pub fn from<S>(self, table: S) -> SelectFrom
    where
        S: ToString,
    {
        let mut builder = SqlBuilder::select_from(table);
        builder.fields(self.0);
        SelectFrom { builder }
    }
}

impl<T, I> Builder for Select<T, I>
where
    T: ToString,
    I: IntoIterator<Item = T>,
{
    fn build(self) -> Result<String> {
        SqlBuilder::select_values(self.0).sql()
    }
}

pub struct SelectFrom {
    builder: SqlBuilder,
}

impl SelectFrom {
    pub fn join<S>(mut self, table: S) -> Self
    where
        S: ToString,
    {
        self.builder.join(table);
        self
    }
    pub fn left_join<S>(mut self, table: S) -> Self
    where
        S: ToString,
    {
        self.builder.left_join(table);
        self
    }

    pub fn right_join<S>(mut self, table: S) -> Self
    where
        S: ToString,
    {
        self.builder.right_join(table);
        self
    }

    pub fn inner_join<S>(mut self, table: S) -> Self
    where
        S: ToString,
    {
        self.builder.inner_join(table);
        self
    }
    pub fn with_where<S: ToString>(mut self, condition: S) -> SelectWhere {
        self.builder.and_where(condition);
        SelectWhere {
            builder: self.builder,
        }
    }
    pub fn group_by<S: ToString>(mut self, column: S) -> GroupBy {
        self.builder.group_by(column);
        GroupBy {
            builder: self.builder,
        }
    }
    pub fn order_by_asc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_asc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn order_by_desc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_desc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn limit(mut self, limit: u32) -> Limit {
        self.builder.limit(limit);
        Limit {
            builder: self.builder,
        }
    }
}

impl Builder for SelectFrom {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct SelectWhere {
    builder: SqlBuilder,
}

impl SelectWhere {
    pub fn and_where<S: ToString>(mut self, condition: S) -> Self {
        self.builder.and_where(condition);
        self
    }
    pub fn or_where<S: ToString>(mut self, condition: S) -> Self {
        self.builder.or_where(condition);
        self
    }
    pub fn group_by<S: ToString>(mut self, column: S) -> GroupBy {
        self.builder.group_by(column);
        GroupBy {
            builder: self.builder,
        }
    }
    pub fn order_by_asc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_asc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn order_by_desc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_desc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn limit(mut self, limit: u32) -> Limit {
        self.builder.limit(limit);
        Limit {
            builder: self.builder,
        }
    }
}

impl Builder for SelectWhere {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct GroupBy {
    builder: SqlBuilder,
}

impl GroupBy {
    pub fn having<S: ToString>(mut self, condition: S) -> Having {
        self.builder.having(condition);
        Having {
            builder: self.builder,
        }
    }
    pub fn order_by_asc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_asc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn order_by_desc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_desc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn limit(mut self, limit: u32) -> Limit {
        self.builder.limit(limit);
        Limit {
            builder: self.builder,
        }
    }
}

impl Builder for GroupBy {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct Having {
    builder: SqlBuilder,
}

impl Having {
    pub fn order_by_asc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_asc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn order_by_desc<S: ToString>(mut self, column: S) -> OrderBy {
        self.builder.order_desc(column);
        OrderBy {
            builder: self.builder,
        }
    }
    pub fn limit(mut self, limit: u32) -> Limit {
        self.builder.limit(limit);
        Limit {
            builder: self.builder,
        }
    }
}

impl Builder for Having {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct OrderBy {
    builder: SqlBuilder,
}

impl OrderBy {
    pub fn limit(mut self, limit: u32) -> Limit {
        self.builder.limit(limit);
        Limit {
            builder: self.builder,
        }
    }
}

impl Builder for OrderBy {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct Limit {
    builder: SqlBuilder,
}

impl Limit {
    pub fn offset(mut self, offset: u32) -> Offset {
        self.builder.offset(offset);
        Offset {
            builder: self.builder,
        }
    }
}

impl Builder for Limit {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

pub struct Offset {
    builder: SqlBuilder,
}

impl Builder for Offset {
    fn build(self) -> Result<String> {
        self.builder.sql()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select() {
        let select = Select(["column1", "column2"]).from("table1");
        assert_eq!(
            select.build().unwrap(),
            "SELECT column1, column2 FROM table1;"
        );

        let select = Select(["column1", "column2"])
            .from("table1")
            .join("table2 ON table1.id = table2.table1_id")
            .with_where("table1.name = 'John'");

        assert_eq!(
            select.build().unwrap(),
            "SELECT column1, column2 FROM table1 JOIN table2 ON table1.id = table2.table1_id WHERE table1.name = 'John';"
        );
    }
}
