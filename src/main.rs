use duplicate_remover::{
    fc_generator::{self, models::SearchOptions},
    fc_processor::operations::show_duplicates,
};
use rusqlite::Result;

fn main() -> Result<()> {
    // fc_generator::hasher::generate_cache(SearchOptions::new());
    // Ok(())
    show_duplicates("files.db").unwrap();
    Ok(())
}
