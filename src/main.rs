use duplicate_remover::{
    fc_generator::{self, models::SearchOptions},
    fc_processor::operations::{remove_by_hash, show_duplicates},
};
use rusqlite::Result;

fn main() -> Result<()> {
    // fc_generator::hasher::generate_cache(SearchOptions::new());
    // Ok(())

    // show_duplicates("files.db").unwrap();
    // Ok(())

    remove_by_hash(
        "files.db",
        "dc148a4ee880ff0beadb6c814b201b0590239d029a8c50eb4b0a7ba0a5983d19",
    )
}
