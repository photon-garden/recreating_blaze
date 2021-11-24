use std::path::Path;

pub fn folder() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}
