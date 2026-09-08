use crate::models::ScanResult;
use crate::report::ReportRenderer;

pub struct MarkdownRenderer;

impl ReportRenderer for MarkdownRenderer {
    fn render(&self, result: &ScanResult) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        let p = &result.project;

        output.push_str(&format!("# StellarPath Analysis Report: {}\n\n", p.name));

        output.push_str("## Project Overview\n");
        output.push_str(&format!(
            "* **Archetype:** {}\n",
            p.archetype.as_deref().unwrap_or("Unknown")
        ));
        output.push_str(&format!("* **Languages:** {}\n", p.languages.join(", ")));
        output.push_str(&format!("* **Scan Duration:** {:.2}s\n\n", p.scan_duration));

        output.push_str("## Detected Components\n");
        if p.evidence.is_empty() {
            output.push_str("No components detected.\n\n");
        } else {
            output.push_str("| Component | Path | Confidence | Reason |\n");
            output.push_str("|-----------|------|------------|--------|\n");
            for e in &p.evidence {
                output.push_str(&format!(
                    "| {:?} | `{}` | {:.2} | {} |\n",
                    e.component_type, e.path, e.confidence, e.reason
                ));
            }
            output.push('\n');
        }

        output.push_str("## Important Locations\n");
        if p.important_files.is_empty() {
            output.push_str("None\n\n");
        } else {
            for f in &p.important_files {
                output.push_str(&format!("* `{}`\n", f));
            }
            output.push('\n');
        }

        output.push_str("## Suggested Starting Path\n");
        if p.recommendations.is_empty() {
            output.push_str("No specific recommendations.\n");
        } else {
            for r in &p.recommendations {
                output.push_str(&format!("### Step {}: {}\n", r.step, r.title));
                output.push_str(&format!("* **Path:** `{}`\n", r.path));
                output.push_str(&format!("* **Reason:** {}\n", r.reason));
                if let Some(ref action) = r.suggested_action {
                    output.push_str(&format!("* **Action:** `{}`\n", action));
                }
                output.push('\n');
            }
        }

        Ok(output)
    }
}
