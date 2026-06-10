//! Shared style-manager list row.
//!
//! Every style manager renders the same left-hand list of style names where a
//! single click selects and a double click renames inline. Only the row's
//! button style differs per manager, so it is passed in.

use crate::app::{Message, StyleKind};
use iced::widget::button::{Status, Style};
use iced::widget::{button, mouse_area, text, text_input};
use iced::{Element, Fill, Theme};

/// One row of the style list. Renders an editable `text_input` when `name` is
/// the style being renamed (`rename_active`), otherwise a selectable button
/// wrapped in a `mouse_area` whose double click starts the rename.
pub fn item<'a>(
    name: &str,
    label: String,
    kind: StyleKind,
    on_select: Message,
    rename_active: Option<&str>,
    rename_buf: &'a str,
    style: impl Fn(&Theme, Status) -> Style + 'a,
) -> Element<'a, Message> {
    if rename_active == Some(name) {
        text_input("", rename_buf)
            .on_input(Message::StyleRenameEdit)
            .on_submit(Message::StyleRenameCommit(kind))
            .size(11)
            .padding([4, 8])
            .width(Fill)
            .into()
    } else {
        mouse_area(
            button(text(label).size(11))
                .on_press(on_select)
                .style(style)
                .padding([4, 8])
                .width(Fill),
        )
        .on_double_click(Message::StyleRenameStart(kind, name.to_string()))
        .into()
    }
}
