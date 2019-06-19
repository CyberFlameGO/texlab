use crate::completion::factory;
use crate::completion::latex::combinators::{self, ArgumentLocation};
use crate::feature::{FeatureProvider, FeatureRequest};
use futures_boxed::boxed;
use lsp_types::{CompletionItem, CompletionParams};
use std::borrow::Cow;
use std::sync::Arc;

#[derive(Debug, PartialEq, Clone)]
pub struct LatexColorCompletionProvider {
    items: Vec<Arc<CompletionItem>>,
}

impl LatexColorCompletionProvider {
    pub fn new() -> Self {
        let items = COLOR_NAMES
            .iter()
            .map(|name| factory::create_color(Cow::from(*name)))
            .map(Arc::new)
            .collect();

        Self { items }
    }
}

impl FeatureProvider for LatexColorCompletionProvider {
    type Params = CompletionParams;
    type Output = Vec<Arc<CompletionItem>>;

    #[boxed]
    async fn execute<'a>(&'a self, request: &'a FeatureRequest<Self::Params>) -> Self::Output {
        let locations = COLOR_COMMANDS
            .iter()
            .map(|cmd| ArgumentLocation::new(cmd, 0));

        combinators::argument(request, locations, async move |_| self.items.clone()).await
    }
}

const COLOR_COMMANDS: &[&str] = &[
    "\\color",
    "\\colorbox",
    "\\textcolor",
    "\\pagecolor",
    "\\colorlet",
    "\\definespotcolor",
];

const COLOR_NAMES: &[&str] = &[
    "black",
    "blue",
    "brown",
    "cyan",
    "darkgray",
    "gray",
    "green",
    "lightgray",
    "lime",
    "magenta",
    "olive",
    "orange",
    "pink",
    "purple",
    "red",
    "teal",
    "violet",
    "white",
    "yellow",
    "Apricot",
    "Bittersweet",
    "Blue",
    "BlueViolet",
    "Brown",
    "CadetBlue",
    "Cerulean",
    "Cyan",
    "DarkOrchid",
    "ForestGreen",
    "Goldenrod",
    "Green",
    "JungleGreen",
    "LimeGreen",
    "Mahogany",
    "Melon",
    "Mulberry",
    "OliveGreen",
    "OrangeRed",
    "Peach",
    "PineGreen",
    "ProcessBlue",
    "RawSienna",
    "RedOrange",
    "Rhodamine",
    "RoyalPurple",
    "Salmon",
    "Sepia",
    "SpringGreen",
    "TealBlue",
    "Turquoise",
    "VioletRed",
    "WildStrawberry",
    "YellowGreen",
    "Aquamarine",
    "Black",
    "BlueGreen",
    "BrickRed",
    "BurntOrange",
    "CarnationPink",
    "CornflowerBlue",
    "Dandelion",
    "Emerald",
    "Fuchsia",
    "Gray",
    "GreenYellow",
    "Lavender",
    "Magenta",
    "Maroon",
    "MidnightBlue",
    "NavyBlue",
    "Orange",
    "Orchid",
    "Periwinkle",
    "Plum",
    "Purple",
    "Red",
    "RedViolet",
    "RoyalBlue",
    "RubineRed",
    "SeaGreen",
    "SkyBlue",
    "Tan",
    "Thistle",
    "Violet",
    "White",
    "Yellow",
    "YellowOrange",
];

#[cfg(test)]
mod tests {
    use super::*;
    use crate::feature::{test_feature, FeatureSpec};
    use lsp_types::Position;

    #[test]
    fn test_inside_color() {
        let items = test_feature(
            LatexColorCompletionProvider::new(),
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", "\\color{}")],
                main_file: "foo.tex",
                position: Position::new(0, 7),
                ..FeatureSpec::default()
            },
        );
        assert!(items.iter().any(|item| item.label == "black"));
    }

    #[test]
    fn test_outside_color() {
        let items = test_feature(
            LatexColorCompletionProvider::new(),
            FeatureSpec {
                files: vec![FeatureSpec::file("foo.tex", "\\color{}")],
                main_file: "foo.tex",
                position: Position::new(0, 8),
                ..FeatureSpec::default()
            },
        );
        assert!(items.is_empty());
    }
}
