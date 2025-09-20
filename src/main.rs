use duplicate_remover::{
    fc_generator::{self, models::SearchOptions},
    fc_processor::operations::{remove_by_hash, show_duplicates},
};
// use duplicate_remover::{
//     fc_generator::{self, models::SearchOptions},
//     fc_processor::operations::{remove_by_hash, show_duplicates},
// };
use rusqlite::Result;

fn main() -> Result<()> {
    // fc_generator::hasher::generate_cache(SearchOptions::new());
    // Ok(())
    let mut s = SearchOptions::new();
    s.set_origin("/home");
    fc_generator::hasher::generate_cache(s);
    //show_duplicates("files.db").unwrap();
    // Ok(())

    // remove_by_hash(
    //     "files.db",
    //     "57503c3b0ff331110019dbe5c54609b76b75313a45368998169d00ff94f1b043",
    // );
    Ok(())
}
