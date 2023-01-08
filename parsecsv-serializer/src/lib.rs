use std::io::Write;

pub trait Serializer {
    fn serialize<'a>(&self, w: &'a mut dyn Write) -> Result<(), std::io::Error>;
}

macro_rules! impl_serializer {
    ($type:ty) => {
        impl Serializer for $type {
            fn serialize<'a>(&self, w: &'a mut dyn Write) -> Result<(), std::io::Error> {
                write!(w, "{}", self)
            }
        }
    };
}

impl_serializer!(u8);
impl_serializer!(u16);
impl_serializer!(u32);
impl_serializer!(u64);
impl_serializer!(u128);
impl_serializer!(usize);

impl_serializer!(i8);
impl_serializer!(i16);
impl_serializer!(i32);
impl_serializer!(i64);
impl_serializer!(i128);
impl_serializer!(isize);

impl_serializer!(f32);
impl_serializer!(f64);

impl_serializer!(bool);
impl_serializer!(char);

macro_rules! impl_serializer_str {
    ($type:ty) => {
        impl Serializer for $type {
            fn serialize<'a>(&self, w: &'a mut dyn Write) -> Result<(), std::io::Error> {
                if self.contains(" ") {
                    write!(w, "\"")?;
                    write!(w, "{}", self)?;
                    write!(w, "\"")?;
                    Ok(())
                } else {
                    write!(w, "{}", self)
                }
            }
        }
    };
}

impl_serializer_str!(str);
impl_serializer_str!(&str);
impl_serializer_str!(String);
impl_serializer_str!(&String);

impl<T> Serializer for [T]
where
    T: Serializer,
{
    fn serialize<'a>(&self, w: &'a mut dyn Write) -> Result<(), std::io::Error> {
        write!(w, "\"")?;
        let size = self.len();
        for (i, e) in self.iter().enumerate() {
            e.serialize(w)?;
            if i < size - 1 {
                write!(w, ",")?;
            }
        }
        write!(w, "\"")?;
        Ok(())
    }
}

impl<T> Serializer for Vec<T>
where
    T: Serializer,
{
    fn serialize<'a>(&self, w: &'a mut dyn Write) -> Result<(), std::io::Error> {
        write!(w, "\"")?;
        let size = self.len();
        for (i, e) in self.iter().enumerate() {
            e.serialize(w)?;
            if i < size - 1 {
                write!(w, ",")?;
            }
        }
        write!(w, "\"")?;
        Ok(())
    }
}
