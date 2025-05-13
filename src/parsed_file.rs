use crate::diagnostic::Diagnostic;

pub struct ParsedFile {
    schema: Option<ProtoSchema>,
    diagnostics: Vec<Diagnostic>,
}

impl ParsedFile {
    pub fn new() -> Self {
        Self {
            schema: None,
            diagnostics: vec![],
        }
    }
}
