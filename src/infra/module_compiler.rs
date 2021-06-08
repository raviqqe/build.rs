use crate::{FilePath, ModulePath};

pub trait ModuleCompiler {
    fn compile_dependencies(
        &self,
        source: &str,
        source_file_path: &FilePath,
    ) -> Result<Vec<ModulePath>, Box<dyn std::error::Error>>;
}
