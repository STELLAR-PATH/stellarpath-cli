use ignore::{overrides::OverrideBuilder, WalkBuilder};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default)]
pub struct ScanConfig {
    pub max_depth: Option<usize>,
    pub follow_symlinks: bool,
}

pub struct Scanner {
    config: ScanConfig,
}

impl Scanner {
    pub fn new(config: ScanConfig) -> Self {
        Self { config }
    }

    pub fn scan<P: AsRef<Path>>(&self, root: P) -> Vec<PathBuf> {
        let mut paths = Vec::new();
        let root_path = root.as_ref();

        let mut builder = WalkBuilder::new(root_path);
        builder.follow_links(self.config.follow_symlinks);
        builder.max_depth(self.config.max_depth);
        builder.add_custom_ignore_filename(".stellarpathignore");

        // Add default exclusions
        let mut overrides = OverrideBuilder::new(root_path);
        let exclusions = [
            "!**/target",
            "!**/.git",
            "!**/node_modules",
            "!**/vendor",
            "!**/dist",
            "!**/build",
            "!**/*.wasm",
        ];
        for exclusion in exclusions {
            let _ = overrides.add(exclusion);
        }
        if let Ok(override_set) = overrides.build() {
            builder.overrides(override_set);
        }

        for result in builder.build() {
            match result {
                Ok(entry) => {
                    let path = entry.path();
                    if path.is_file() {
                        let rel_path = path.strip_prefix(root_path).unwrap_or(path);
                        if rel_path.as_os_str().is_empty() {
                            continue;
                        }
                        paths.push(rel_path.to_path_buf());
                    }
                }
                Err(_) => {
                    // Gracefully ignore permission denied and other errors
                }
            }
        }

        paths
    }
}
