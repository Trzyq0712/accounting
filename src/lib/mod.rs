use std::path::Path;

pub const OPERATIONS: [Ops; 3] = [Ops::Kurier, Ops::Allegro, Ops::FTS];

#[derive(Copy, Clone, PartialEq, Eq, Debug, Hash)]
pub enum Ops {
    Kurier,
    Allegro,
    FTS,
}

impl Ops {
    pub fn into_selector_str(self) -> &'static str {
        use Ops::*;
        match self {
            Kurier => "operation.kurier",
            Allegro => "operation.allegro",
            FTS => "operation.fts",
        }
    }
    pub fn process<F: AsRef<Path>>(&self, file: F) {
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


pub fn process_allegro<F: AsRef<Path>>(file: F) -> Result<(), String> {

    Ok(())
}
