use std::path::Path;

pub trait Repository {
    fn root(&self) -> &Path;
}
