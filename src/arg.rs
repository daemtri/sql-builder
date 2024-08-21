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

pub struct NULL;

impl SqlArg for NULL {
    fn sql_arg(&self) -> String {
        String::from("NULL")
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
                format!("({})", [$($name.sql_arg(),)*].join(", "))
            }
        }

        impl<$($type: SqlArg),*> SqlArg for &($($type,)*) {
            fn sql_arg(&self) -> String {
                let ($($name,)*) = self;
                format!("({})", [$($name.sql_arg(),)*].join(", "))
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

impl<T: SqlArg, const N: usize> SqlArg for [T; N] {
    fn sql_arg(&self) -> String {
        let res = self
            .iter()
            .map(|item| item.sql_arg())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", res)
    }
}

impl<T: SqlArg, const N: usize> SqlArg for &[T; N] {
    fn sql_arg(&self) -> String {
        let res = self
            .iter()
            .map(|item| item.sql_arg())
            .collect::<Vec<_>>()
            .join(", ");
        format!("({})", res)
    }
}

pub trait SqlArgs {
    fn sql_args(&self) -> Vec<String>;
}

macro_rules! impl_sql_args_tuple {
    ($($type:ident : $name:ident),*) => {
        impl<$($type: SqlArg),*> SqlArgs for ($($type,)*) {
            fn sql_args(&self) -> Vec<String> {
                let ($($name,)*) = self;
                let mut args = Vec::new();
                $(args.push($name.sql_arg());)*
                args
            }
        }

        impl<$($type: SqlArg),*> SqlArgs for &($($type,)*) {
            fn sql_args(&self) -> Vec<String> {
                let ($($name,)*) = self;
                let mut args = Vec::new();
                $(args.push($name.sql_arg());)*
                args
            }
        }
    };
}

impl_sql_args_tuple!(A:a);
impl_sql_args_tuple!(A:a, B:b);
impl_sql_args_tuple!(A:a, B:b, C:c);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j, K:k);
impl_sql_args_tuple!(A:a, B:b, C:c, D:d, E:e, F:f, G:g, H:h, I:i, J:j, K:k, L:l);

impl<const N: usize> SqlArgs for [&dyn SqlArg; N] {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl SqlArgs for &[&dyn SqlArg] {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl SqlArgs for Vec<&dyn SqlArg> {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl SqlArgs for Vec<Box<dyn SqlArg>> {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl<const N: usize> SqlArgs for [Box<dyn SqlArg>; N] {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl SqlArgs for &[Box<dyn SqlArg>] {
    fn sql_args(&self) -> Vec<String> {
        self.iter().map(|arg| arg.sql_arg()).collect()
    }
}

impl<const N: usize> SqlArgs for [Option<&dyn SqlArg>; N] {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}

impl SqlArgs for &[Option<&dyn SqlArg>] {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}

impl SqlArgs for Vec<Option<&dyn SqlArg>> {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}

impl SqlArgs for Vec<Option<Box<dyn SqlArg>>> {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}

impl<const N: usize> SqlArgs for [Option<Box<dyn SqlArg>>; N] {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}

impl SqlArgs for &[Option<Box<dyn SqlArg>>] {
    fn sql_args(&self) -> Vec<String> {
        self.iter()
            .map(|arg| {
                arg.as_ref()
                    .map(|arg| arg.sql_arg())
                    .unwrap_or_else(|| "NULL".into())
            })
            .collect()
    }
}
