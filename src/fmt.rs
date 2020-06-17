use crate::datatypes::{AnyType, ToStr};
use crate::{
    frame::DataFrame,
    series::{chunked_array::iterator::ChunkIterator, series::Series},
};
use std::{
    fmt,
    fmt::{Debug, Display, Formatter},
};

impl Debug for Series {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        const LIMIT: usize = 10;

        macro_rules! format_series {
            ($a:ident, $name:expr) => {{
                write![f, "Series: {} \n[\n", $name]?;
                $a.iter().take(LIMIT).for_each(|v| {
                    match v {
                        Some(v) => {
                            write!(f, "\t{}\n", v).ok();
                        }
                        None => {
                            write!(f, "\tnull").ok();
                        }
                    };
                });
                write![f, "]"]
            }};
        }

        match self {
            Series::Int32(a) => format_series!(a, "i32"),
            Series::Int64(a) => format_series!(a, "i64"),
            Series::UInt32(a) => format_series!(a, "u32"),
            Series::Bool(a) => format_series!(a, "bool"),
            Series::Float32(a) => format_series!(a, "f32"),
            Series::Float64(a) => format_series!(a, "f64"),
            Series::Utf8(a) => {
                write![f, "Series: str \n[\n"]?;
                a.iter().take(LIMIT).for_each(|v| {
                    write!(f, "\t{}\n", &v[..LIMIT]).ok();
                });
                write![f, "]"]
            }
            _ => write!(f, "hello"),
        }
    }
}

impl Display for Series {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Debug for DataFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl Display for DataFrame {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for field in self.schema.fields() {
            write!(f, "{:>15}", field.name())?;
        }
        write!(f, "\n")?;
        for field in self.schema.fields() {
            write!(f, "{:>15}", field.data_type().to_str())?;
        }
        write!(f, "\n")?;
        for _ in self.schema.fields() {
            write!(f, "{:>15}", "---")?;
        }
        write!(f, "\n\n")?;

        for i in 0..10 {
            let opt = self.get(i);
            if let Some(row) = opt {
                for v in row {
                    write!(f, "{}", v)?;
                }
                write!(f, "\n")?;
            }
        }
        Ok(())
    }
}

impl Display for AnyType<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let width = 15;
        match self {
            AnyType::Null => write!(f, "{:width$}", "null", width = width),
            AnyType::U32(v) => write!(f, "{:width$}", v, width = width),
            AnyType::I32(v) => write!(f, "{:width$}", v, width = width),
            AnyType::I64(v) => write!(f, "{:width$}", v, width = width),
            AnyType::F32(v) => write!(f, "{:width$}", v, width = width),
            AnyType::F64(v) => write!(f, "{:width$}", v, width = width),
            AnyType::Bool(v) => write!(f, "{:width$}", v, width = width),
            AnyType::Str(v) => write!(f, "{:width$}", v, width = width),
        }
    }
}