import re

css = """
    .bg-code {
      position: fixed;
      top: 0;
      left: -10vw;
      width: 120vw;
      height: 200vh;
      z-index: -2;
      color: rgba(255, 179, 0, 0.04);
      font-family: "JetBrains Mono", monospace;
      font-size: 14px;
      line-height: 1.6;
      white-space: pre;
      overflow: hidden;
      pointer-events: none;
      animation: scrollCode 60s linear infinite;
    }
    @keyframes scrollCode {
      0% { transform: translateY(0); }
      100% { transform: translateY(-50%); }
    }
"""

code_snippet = """use crate::models::{ComponentType, Evidence};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub struct LintResult {
    pub findings: Vec<Evidence>,
}

pub fn run_lint(root_path: &Path) -> Result<LintResult, Box<dyn std::error::Error>> {
    let mut findings = Vec::new();
    let mut files = Vec::new();
    
    for entry in WalkDir::new(root_path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            files.push(entry.path().to_path_buf());
        }
    }

    for file in &files {
        if file.extension().and_then(|e| e.to_str()) == Some("rs") {
            if let Ok(content) = fs::read_to_string(file) {
                if !content.contains("#[contract]") && !content.contains("#[contractimpl]") {
                    continue;
                }

                let lines: Vec<&str> = content.lines().collect();
                for (i, line) in lines.iter().enumerate() {
                    if line.contains("panic!(") && !line.contains("panic_with_error!(") {
                        findings.push(Evidence {
                            component_type: ComponentType::SorobanContract,
                            path: file.strip_prefix(root_path).unwrap_or(file).to_string_lossy().to_string(),
                            detector_name: "SecurityLint".to_string(),
                            reason: format!("Line {}: Mistake #17", i + 1),
                            confidence: 1.0,
                        });
                    }
                }
            }
        }
    }
    Ok(LintResult { findings })
}
"""
repeated_code = (code_snippet * 10).replace("<", "&lt;").replace(">", "&gt;")

html_to_inject = f"""  <div class="bg-code" aria-hidden="true">
{repeated_code}
  </div>"""

with open('website/index.html', 'r') as f:
    content = f.read()

# Insert CSS just before </style>
content = content.replace('</style>', css + '</style>')

# Insert HTML just after <div class="ambient-light"></div>
content = content.replace('<div class="ambient-light"></div>', '<div class="ambient-light"></div>\n' + html_to_inject)

with open('website/index.html', 'w') as f:
    f.write(content)
