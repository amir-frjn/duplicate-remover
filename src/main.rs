use duplicate_remover::fc_processor::operations::show_duplicates;
use rusqlite::Result;

fn main() -> Result<()> {
    show_duplicates("files.db").unwrap();
    Ok(())
}
