use std::{env, fs};

use regex::Regex;

use crate::utils::logging::Logging;

pub struct Includes {}

impl Includes {
    const IMPORT_EXPRESSION: &'static str = r#"^@include\s["<].*[">]"#;
    const STD_EXPRESSION: &'static str = r"<.*>";
    pub fn resolve_imports(contents: String, path_to_std: &str) -> String {
        let mut resolved_imports: Vec<String> = Vec::new();
        let mut new_contents = contents.clone();
        let lines = contents.split("\n");
        for (index, line) in lines.enumerate() {
            // if the line is an import
            if Regex::new(Self::IMPORT_EXPRESSION).unwrap().is_match(&line) {
                Self::resolve_import(
                    line,
                    index,
                    &mut resolved_imports,
                    &mut new_contents,
                    path_to_std,
                );
            }
        }

        // if we have an import then we must have a main entry point
        if resolved_imports.len() > 0 {
            new_contents.insert_str(0, "lda [main]\njnz 1\n");
        }

        return new_contents;
    }

    fn resolve_import(
        line: &str,
        line_num: usize,
        resolved_imports: &mut Vec<String>,
        contents: &mut String,
        path_to_std: &str,
    ) {
        let mut parts = line.splitn(2, " ");
        parts.next();
        let path = parts.next().unwrap().trim();

        if resolved_imports.contains(&path.to_string()) {
            *contents = contents.replacen(line, "", 1);
            Logging::log_import_error("Duplicate imports found", line_num, line);
            return;
        }

        let is_std = Regex::new(Self::STD_EXPRESSION).unwrap().is_match(path);

        let import_replacement: Option<String>;
        if is_std {
            let trimmed_path = path
                .strip_prefix("<")
                .unwrap_or(path)
                .strip_suffix(">")
                .unwrap_or(path);

            import_replacement = Self::std_import(trimmed_path, path_to_std);
        } else {
            let trimmed_path = path
                .strip_prefix("\"")
                .unwrap_or(path)
                .strip_suffix("\"")
                .unwrap_or(path);

            import_replacement = Self::custom_import(trimmed_path);
        }

        if import_replacement.is_none() {
            Logging::log_import_error("Unable to resolve import", line_num, line);
            return;
        }
        let import_replacement = import_replacement.unwrap();

        *contents = contents.replacen(line, &import_replacement, 1);
        resolved_imports.push(path.to_string());
    }

    fn std_import(path: &str, path_to_std: &str) -> Option<String> {
        let file = fs::read_to_string(format!("{}/{}", path_to_std, path));

        return file.ok();
    }
    fn custom_import(path: &str) -> Option<String> {
        let file = fs::read_to_string(format!("./asm/{}", path));
        return file.ok();
    }
}
