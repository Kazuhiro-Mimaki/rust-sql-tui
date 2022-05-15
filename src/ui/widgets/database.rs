use tui::{
    style::Style,
    widgets::{Block, Borders, Paragraph},
};

pub struct DatabaseWdg<'a> {
    title: &'a str,
    content: &'a str,
}

impl DatabaseWdg<'_> {
    pub fn new() -> Self {
        Self {
            title: "DB",
            content: "sample database",
        }
    }

    pub fn widget(&self) -> Paragraph<'static> {
        let block = Block::default()
            .title(self.title.to_string())
            .borders(Borders::ALL);
        let widget = Paragraph::new(self.content.to_string())
            .style(Style::default())
            .block(block);
        return widget;
    }
}
