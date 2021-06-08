use super::package_configuration::PackageConfiguration;
use crate::{
    infra::{FileSystem, ModuleCompiler},
    BuildError, FilePath, ModulePath,
};
use petgraph::{algo::toposort, graph::Graph};
use std::collections::HashMap;

pub struct ModuleSetBuilder {
    module_compiler: Box<dyn ModuleCompiler>,
    file_system: Box<dyn FileSystem>,
}

impl ModuleSetBuilder {
    pub fn new(module_compiler: Box<dyn ModuleCompiler>, file_system: Box<dyn FileSystem>) -> Self {
        Self {
            module_compiler,
            file_system,
        }
    }

    pub fn build<MI>(
        &self,
        package_configuration: &PackageConfiguration,
        external_module_interfaces: &HashMap<ModulePath, MI>,
        prelude_module_interfaces: &[MI],
    ) -> Result<(Vec<FilePath>, Vec<FilePath>), Box<dyn std::error::Error>> {
        let mut module_interfaces = external_module_interfaces
            .iter()
            .map(|(path, module_interface)| (path.clone().into(), module_interface.clone()))
            .collect::<HashMap<ModulePath, MI>>();

        let mut object_file_paths = vec![];
        let mut interface_file_paths = vec![];

        for source_file_path in self.sort_source_file_paths(
            &self
                .modules_finder
                .find(package_configuration.directory_path())?,
            package_configuration,
        )? {
            let (object_file_path, interface_file_path) = self.module_compiler.compile(
                source_file_path,
                &module_interfaces,
                prelude_module_interfaces,
                package_configuration,
            )?;

            let module_interface = serde_json::from_str::<MI>(
                &self.file_system.read_to_string(&interface_file_path)?,
            )?;
            module_interfaces.insert(
                module_interface.path().internal_unresolved().into(),
                module_interface,
            );

            object_file_paths.push(object_file_path);
            interface_file_paths.push(interface_file_path);
        }

        Ok((object_file_paths, interface_file_paths))
    }

    fn sort_source_file_paths(
        &self,
        source_file_paths: &[FilePath],
        package_configuration: &PackageConfiguration,
    ) -> Result<Vec<&FilePath>, Box<dyn std::error::Error>> {
        let mut graph = Graph::<&FilePath, ()>::new();
        let mut indices = HashMap::<&FilePath, _>::new();

        for source_file_path in source_file_paths {
            indices.insert(source_file_path, graph.add_node(source_file_path));
        }

        for source_file_path in source_file_paths {
            for module_path in self.module_compiler.compile_dependencies(
                &self.file_system.read_to_string(source_file_path)?,
                source_file_path,
            )? {
                if let Some(&index) = indices.get(&module_path) {
                    graph.add_edge(index, indices[&source_file_path], ());
                }
            }
        }

        Ok(toposort(&graph, None)
            .map_err(|cycle| BuildError::ModuleCircularDependency(graph[cycle.node_id()].clone()))?
            .into_iter()
            .map(|index| graph[index])
            .collect())
    }
}
