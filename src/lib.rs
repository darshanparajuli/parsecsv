extern crate parsecsv_proc_macro;

extern crate parsecsv_serializer;

pub use parsecsv_proc_macro::CsvSerialize;
pub use parsecsv_serializer::Serializer;

pub fn to_string<T>(data: &T) -> Result<String, std::io::Error>
where
    T: Serializer,
{
    let mut writer = std::io::BufWriter::new(Vec::new());

    data.serialize(&mut writer)?;

    Ok(String::from_utf8_lossy(&writer.into_inner()?).into())
}
