use std::path::Path;

use druid::Selector;
pub mod allegro;

pub const OPERATIONS: [Ops; 3] = [Ops::Kurier, Ops::Allegro, Ops::FTS];

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Ops {
    Kurier,
    Allegro,
    FTS,
}

impl Ops {
    pub fn into_selector(self) -> Selector {
        use Ops::*;
        let op_str = match self {
            Kurier => "operation.kurier",
            Allegro => "operation.allegro",
            FTS => "operation.fts",
        };
        Selector::new(op_str)
    }
}

impl Into<&str> for Ops {
    fn into(self) -> &'static str {
        use Ops::*;
        match self {
            Kurier => "Kurier",
            Allegro => "Allegro",
            FTS => "FTS",
        }
    }
}

pub trait Processor {
    fn process<P: AsRef<Path>>(&self, path: P) -> Result<csv::Writer<Vec<u8>>, csv::Error>;
}
