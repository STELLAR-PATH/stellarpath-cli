use crate::detector::{Detector, ScanContext};
use crate::models::{ComponentType, Evidence};
use std::fs;
use std::path::Path;
use toml::Table;

pub fn has_soroban_sdk_dependency(manifest_path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(manifest_path) {
        if let Ok(table) = content.parse::<Table>() {
            if let Some(dependencies) = table.get("dependencies").and_then(|v| v.as_table()) {
                if dependencies.contains_key("soroban-sdk") {
                    return true;
                }
            }
            if let Some(dev_dependencies) = table.get("dev-dependencies").and_then(|v| v.as_table())
            {
                if dev_dependencies.contains_key("soroban-sdk") {
                    return true;
                }
            }
        }
    }
    false
}

fn check_attrs(attrs: &[syn::Attribute]) -> bool {
    attrs.iter().any(|attr| {
        attr.meta
            .path()
            .get_ident()
            .map(|i| {
                let name = i.to_string();
                name == "contract" || name == "contractimpl"
            })
            .unwrap_or(false)
    })
}

pub fn has_contract_attribute(file_path: &Path) -> bool {
    if let Ok(content) = fs::read_to_string(file_path) {
        if let Ok(file) = syn::parse_file(&content) {
            for item in &file.items {
                match item {
                    syn::Item::Struct(s) if check_attrs(&s.attrs) => return true,
                    syn::Item::Impl(i) if check_attrs(&i.attrs) => return true,
                    syn::Item::Fn(f) if check_attrs(&f.attrs) => return true,
                    _ => {}
                }
            }
        }
    }
    false
}

pub struct SorobanDetector;

impl Detector for SorobanDetector {
    fn name(&self) -> &'static str {
        "SorobanDetector"
    }

    fn detect(&self, ctx: &ScanContext) -> Result<Vec<Evidence>, Box<dyn std::error::Error>> {
        let mut evidence = Vec::new();
        let mut has_soroban_dep = false;

        for file in ctx.files {
            if file.file_name().and_then(|n| n.to_str()) == Some("Cargo.toml") {
                let full_path = ctx.root_path.join(file);
                if has_soroban_sdk_dependency(&full_path) {
                    has_soroban_dep = true;
                }
            }
        }

        for file in ctx.files {
            if file.extension().and_then(|e| e.to_str()) == Some("rs") {
                let full_path = ctx.root_path.join(file);
                if has_contract_attribute(&full_path) {
                    let confidence = if has_soroban_dep { 1.0 } else { 0.9 };
                    evidence.push(Evidence {
                        component_type: ComponentType::SorobanContract,
                        path: file.to_string_lossy().to_string(),
                        detector_name: self.name().to_string(),
                        reason: "Found #[contract] or #[contractimpl] attribute in Rust file"
                            .to_string(),
                        confidence,
                    });
                }
            }
        }

        Ok(evidence)
    }
}
