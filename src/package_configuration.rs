#[derive(Clone, Debug)]
pub struct PackageConfiguration {
    package: Package,
    dependencies: HashMap<String, Package>,
    directory_path: FilePath,
}
