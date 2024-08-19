use crate::quote;
use std::borrow::{Cow, ToOwned};

pub trait SqlArg {
    fn sql_arg(&self) -> String;
}

impl SqlArg for &dyn SqlArg {
    fn sql_arg(&self) -> String {
        (**self).sql_arg()
    }
}

impl SqlArg for Box<dyn SqlArg> {
    fn sql_arg(&self) -> String {
        (**self).sql_arg()
    }
}

impl SqlArg for str {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for &str {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for &&str {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for Cow<'_, str> {
    fn sql_arg(&self) -> String {
        quote(self[..].to_owned())
    }
}

impl SqlArg for String {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for &String {
    fn sql_arg(&self) -> String {
        quote(self)
    }
}

impl SqlArg for i8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u8 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u16 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for i128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &i128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for u128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &u128 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for isize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &isize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for usize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &usize {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for f32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &f32 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for f64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for &f64 {
    fn sql_arg(&self) -> String {
        self.to_string()
    }
}

impl SqlArg for bool {
    fn sql_arg(&self) -> String {
        String::from(if *self { "TRUE" } else { "FALSE" })
    }
}

impl SqlArg for &bool {
    fn sql_arg(&self) -> String {
        String::from(if **self { "TRUE" } else { "FALSE" })
    }
}

impl<T: SqlArg> SqlArg for Option<T> {
    fn sql_arg(&self) -> String {
        match &*self {
            Some(value) => value.sql_arg(),
            None => String::from("NULL"),
        }
    }
}

impl<T: SqlArg> SqlArg for &Option<T> {
    fn sql_arg(&self) -> String {
        match &**self {
            Some(value) => value.sql_arg(),
            None => String::from("NULL"),
        }
    }
}

impl<T: SqlArg> SqlArg for Vec<T> {
    fn sql_arg(&self) -> String {
        self.iter()
            .map(|v| v.sql_arg())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<T: SqlArg> SqlArg for &Vec<T> {
    fn sql_arg(&self) -> String {
        self.iter()
            .map(|v| v.sql_arg())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<T: SqlArg> SqlArg for [T] {
    fn sql_arg(&self) -> String {
        self.iter()
            .map(|v| v.sql_arg())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl<T: SqlArg> SqlArg for &[T] {
    fn sql_arg(&self) -> String {
        self.iter()
            .map(|v| v.sql_arg())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

macro_rules! impl_sql_arg_tuple {
    ($($type:ident : $name:ident),*) => {
        impl<$($type: SqlArg),*> SqlArg for ($($type,)*) {
            fn sql_arg(&self) -> String {
                let ($($name,)*) = self;
                [$($name.sql_arg(),)*].join(", ")
            }
        }

        impl<$($type: SqlArg),*> SqlArg for &($($type,)*) {
            fn sql_arg(&self) -> String {
                let ($($name,)*) = self;
                [$($name.sql_arg(),)*].join(", ")
            }
        }
    };
}

impl_sql_arg_tuple!(A:a, B:b);
impl_sql_arg_tuple!(A:a, B:b, C:c);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j, K:k);
impl_sql_arg_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j, K:k, L:l);

macro_rules! impl_sql_arg_array {
    ($len:expr) => {
        impl<T: SqlArg> SqlArg for [T; $len] {
            fn sql_arg(&self) -> String {
                self.iter()
                    .map(|item| item.sql_arg())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }

        impl<T: SqlArg> SqlArg for &[T; $len] {
            fn sql_arg(&self) -> String {
                self.iter()
                    .map(|item| item.sql_arg())
                    .collect::<Vec<_>>()
                    .join(", ")
            }
        }
    };
}

// 为常见的数组长度实现
impl_sql_arg_array!(1);
impl_sql_arg_array!(2);
impl_sql_arg_array!(3);
impl_sql_arg_array!(4);
impl_sql_arg_array!(5);
impl_sql_arg_array!(6);
impl_sql_arg_array!(7);
impl_sql_arg_array!(8);
impl_sql_arg_array!(9);
impl_sql_arg_array!(10);
impl_sql_arg_array!(11);
impl_sql_arg_array!(12);
