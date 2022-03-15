use std::path::PathBuf;

pub struct SourceFile<P> {
    source_file_path: P,
}

impl<P> SourceFile<P>
where
    P: Into<PathBuf>,
{
    pub fn new(source_file_path: P) -> Self {
        SourceFile { source_file_path }
    }
}
