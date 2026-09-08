use crate::models::ScanResult;
use crate::report::ReportRenderer;
use colored::Colorize;

pub struct TerminalRenderer;

impl ReportRenderer for TerminalRenderer {
    fn render(&self, result: &ScanResult) -> Result<String, Box<dyn std::error::Error>> {
        let mut output = String::new();
        let p = &result.project;

        output.push_str(&format!(
            "{}\n",
            "StellarPath Analysis Report".bold().underline()
        ));
        output.push_str(&format!("Name: {}\n", p.name.cyan()));
        let archetype = p.archetype.as_deref().unwrap_or("Unknown");
        output.push_str(&format!("Archetype: {}\n", archetype.yellow()));
        output.push_str(&format!(
            "Languages: {}\n",
            p.languages.join(", ").magenta()
        ));
        output.push_str(&format!("Duration: {:.2}s\n\n", p.scan_duration));

        output.push_str(&format!("{}\n", "Detected Components:".bold()));
        if p.evidence.is_empty() {
            output.push_str("  None detected.\n");
        } else {
            for e in &p.evidence {
                output.push_str(&format!(
                    "  {} {} - {}\n",
                    "✓".green(),
                    e.path.cyan(),
                    e.reason
                ));
            }
        }
        output.push('\n');

        output.push_str(&format!("{}\n", "Important Locations:".bold()));
        if p.important_files.is_empty() {
            output.push_str("  None\n");
        } else {
            for f in &p.important_files {
                output.push_str(&format!("  - {}\n", f.blue()));
            }
        }
        output.push('\n');

        output.push_str(&format!("{}\n", "Suggested Starting Path:".bold()));
        if p.recommendations.is_empty() {
            output.push_str("  No specific recommendations.\n");
        } else {
            for r in &p.recommendations {
                output.push_str(&format!(
                    "  Step {}: {} ({})\n",
                    r.step,
                    r.title.bold(),
                    r.path.cyan()
                ));
                output.push_str(&format!("    Reason: {}\n", r.reason));
                if let Some(ref action) = r.suggested_action {
                    output.push_str(&format!("    Action: {}\n", action.green()));
                }
            }
        }

        Ok(output)
    }
}
