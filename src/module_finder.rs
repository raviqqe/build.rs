use crate::{infra::FileSystem, FilePath, FilePathConfiguration};
use std::sync::Arc;

pub struct ModuleFinder {
    file_system: Arc<dyn FileSystem>,
    file_path_configuration: Arc<FilePathConfiguration>,
}

impl ModuleFinder {
    pub fn new(
        file_system: Arc<dyn FileSystem>,
        file_path_configuration: Arc<FilePathConfiguration>,
    ) -> Self {
        Self {
            file_system,
            file_path_configuration,
        }
    }

    pub fn find(
        &self,
        directory_path: &FilePath,
    ) -> Result<Vec<FilePath>, Box<dyn std::error::Error>> {
        let mut source_file_paths = vec![];

        for path in self.file_system.read_directory(directory_path)? {
            if path
                .relative_to(directory_path)
                .components()
                .next()
                .unwrap()
                .starts_with('.')
            {
            } else if self.file_system.is_directory(&path) {
                source_file_paths.extend(self.find(&path)?);
            } else if path.has_extension(self.file_path_configuration.source_file_extension) {
                source_file_paths.push(path);
            }
        }

        Ok(source_file_paths)
    }
}
