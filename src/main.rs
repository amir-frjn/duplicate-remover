use duplicate_remover::fc_generator::hasher::generate_cache;
use duplicate_remover::fc_generator::models::SearchOptions;

fn main() {
    let search_options = SearchOptions::new();
    generate_cache(search_options).unwrap();
}
