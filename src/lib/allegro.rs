use super::Processor;
use csv;
use serde::de::Deserializer;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct AllegroRecord {
    #[serde(
        rename(deserialize = "kupujący"),
        deserialize_with = "deserialize_first"
    )]
    buyer: String,
    #[serde(rename(deserialize = "oferta"), deserialize_with = "deserialize_first")]
    offer: String,
    #[serde(rename(deserialize = "kwota"), deserialize_with = "deserialize_amount")]
    amount: f64,
}

pub struct AllegroProcessor;

impl Processor for AllegroProcessor {
    fn process<P: AsRef<std::path::Path>>(
        &self,
        path: P,
    ) -> Result<csv::Writer<Vec<u8>>, csv::Error> {
        let mut reader = csv::Reader::from_path(path)?;
        let maybe_records = reader.deserialize::<AllegroRecord>().skip(1);
        let mut wtr = csv::WriterBuilder::new();
        wtr.delimiter(b'\t');
        wtr.has_headers(false);
        let mut wtr = wtr.from_writer(vec![]);
        for r in maybe_records {
            wtr.serialize(r?)?;
        }
        Ok(wtr)
    }
}

fn deserialize_first<'de, D: Deserializer<'de>>(d: D) -> Result<String, D::Error> {
    let s: String = Deserialize::deserialize(d)?;
    let first = s
        .split(';')
        .next()
        .expect("Cannot get first string")
        .to_owned();
    Ok(first)
}

fn deserialize_amount<'de, D: Deserializer<'de>>(d: D) -> Result<f64, D::Error> {
    let s: String = Deserialize::deserialize(d)?;
    let amount = s
        .split(' ')
        .next()
        .expect("Cannot get first string")
        .parse()
        .expect("Cannot parse amount");
    Ok(amount)
}

#[cfg(test)]
mod tests {
    use super::AllegroProcessor;
}
