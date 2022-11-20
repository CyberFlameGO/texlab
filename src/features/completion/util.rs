use lsp_types::{CompletionItemKind, Documentation, MarkupContent, MarkupKind};
use smol_str::SmolStr;

use crate::util::cursor::CursorContext;

pub fn component_detail(file_names: &[SmolStr]) -> String {
    if file_names.is_empty() {
        "built-in".to_owned()
    } else {
        file_names.join(", ")
    }
}

pub fn image_documentation(
    context: &CursorContext,
    name: &str,
    image: &str,
) -> Option<Documentation> {
    if supports_images(context) {
        Some(Documentation::MarkupContent(MarkupContent {
            kind: MarkupKind::Markdown,
            value: format!(
                "![{}](data:image/png;base64,{}|width=48,height=48)",
                name, image
            ),
        }))
    } else {
        None
    }
}

fn supports_images(context: &CursorContext) -> bool {
    context
        .workspace
        .client_capabilities(context.db)
        .text_document
        .as_ref()
        .and_then(|cap| cap.completion.as_ref())
        .and_then(|cap| cap.completion_item.as_ref())
        .and_then(|cap| cap.documentation_format.as_ref())
        .map_or(true, |formats| formats.contains(&MarkupKind::Markdown))
}

pub fn adjust_kind(context: &CursorContext, kind: CompletionItemKind) -> CompletionItemKind {
    if let Some(value_set) = context
        .workspace
        .client_capabilities(context.db)
        .text_document
        .as_ref()
        .and_then(|cap| cap.completion.as_ref())
        .and_then(|cap| cap.completion_item_kind.as_ref())
        .and_then(|cap| cap.value_set.as_ref())
    {
        if value_set.contains(&kind) {
            return kind;
        }
    }
    CompletionItemKind::TEXT
}
