use crate::{
    config,
    finder::{external::ExternalProgram, finder_stream::INDEX_SEPARATOR, FinderStream},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Fuzzel {
    line_length: usize,
    menu_length: usize,
    menu_prompt: String,
    extra_arguments: Vec<String>,
}

impl From<config::Fuzzel> for Fuzzel {
    fn from(
        config::Fuzzel { line_length, menu_length, menu_prompt, extra_arguments }: config::Fuzzel,
    ) -> Self {
        Self { line_length, menu_length, menu_prompt, extra_arguments }
    }
}

impl ExternalProgram for Fuzzel {
    fn program(&self) -> String { "fuzzel".to_string() }

    fn args(&self, _selection_mode: crate::finder::SelectionMode) -> Vec<String> {
        [
            "--dmenu".to_owned(),
            "--width".to_owned(),
            self.menu_length.to_string(),
            "--prompt".to_owned(),
            self.menu_prompt.clone(),
        ]
        .into_iter()
        .chain(self.extra_arguments.clone())
        .collect()
    }
}

impl FinderStream for Fuzzel {
    fn set_extra_arguments(&mut self, arguments: &[String]) {
        self.extra_arguments = arguments.to_vec();
    }

    fn set_line_length(&mut self, line_length: usize) { self.line_length = line_length; }

    fn set_menu_length(&mut self, menu_length: usize) { self.menu_length = menu_length; }

    fn parse_output(&self, data: &[u8]) -> Vec<usize> {
        String::from_utf8_lossy(data)
            .split(INDEX_SEPARATOR)
            .next()
            .expect("first part must exist")
            .parse::<usize>()
            .ok()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        config,
        finder::{
            external::{ExternalProgram, Fuzzel},
            SelectionMode,
        },
    };

    #[test]
    fn test_args() {
        let menu_length = 30;
        let menu_prompt = clipcat_base::DEFAULT_MENU_PROMPT.to_owned();
        let config = config::Fuzzel {
            line_length: 40,
            menu_length,
            menu_prompt,
            extra_arguments: Vec::new(),
        };
        let fuzzel = Fuzzel::from(config.clone());
        assert_eq!(
            fuzzel.args(SelectionMode::Single),
            vec![
                "--dmenu".to_string(),
                "--width".to_string(),
                config.menu_length.to_string(),
                "--prompt".to_string(),
                config.menu_prompt,
            ]
        );
    }
}
