

pub trait JSONFileIO {
    fn write_to_file(&self, path: &std::path::Path) -> Result<i32, String>;
    fn read_from_file(path: &std::path::Path) -> Self;
}
