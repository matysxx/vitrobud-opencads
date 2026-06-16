//! Shared in-canvas modal overlay.
//!
//! Former pop-up *windows* (layer manager, style editors, About, …) render as
//! centered overlays on top of the main view instead of separate OS windows.
//! The native build has one main window and the web build has only the canvas,
//! so both stack dialogs here — one code path for every platform.

use crate::app::Message;
use iced::widget::{button, center, column, container, opaque, stack, text};
use iced::{Background, Border, Color, Element, Theme};

const PANEL: Color = Color {
    r: 0.13,
    g: 0.13,
    b: 0.13,
    a: 1.0,
};
const BORDER_C: Color = Color {
    r: 0.35,
    g: 0.35,
    b: 0.35,
    a: 1.0,
};

/// Stack `content` centered over `base` behind a dimmed backdrop, framed with a
/// close (✕) button in the top-right corner. The backdrop only dims and blocks
/// clicks from reaching the view beneath — it does **not** dismiss the dialog;
/// closing is the ✕ button alone (`on_close`).
pub fn modal<'a>(
    base: impl Into<Element<'a, Message>>,
    content: impl Into<Element<'a, Message>>,
    on_close: Message,
) -> Element<'a, Message> {
    let close = button(text("✕").size(15))
        .on_press(on_close)
        .padding([1, 7])
        .style(close_style);

    // Close button right-aligned above the content. The column shrinks to the
    // content's width (no `Fill`), so the dialog sizes to its content; the ✕ is
    // pushed to that width's right edge.
    let framed = container(
        column![close, content.into()]
            .spacing(6)
            .align_x(iced::alignment::Horizontal::Right),
    )
    .padding(10)
    .style(|_: &Theme| container::Style {
        background: Some(Background::Color(PANEL)),
        border: Border {
            color: BORDER_C,
            width: 1.0,
            radius: 6.0.into(),
        },
        ..Default::default()
    });

    stack![
        base.into(),
        // `opaque` blocks pointer events from passing through, so the dimmed
        // backdrop swallows clicks instead of closing or hitting the view.
        opaque(center(framed).style(|_: &Theme| container::Style {
            background: Some(Background::Color(Color {
                a: 0.55,
                ..Color::BLACK
            })),
            ..Default::default()
        })),
    ]
    .into()
}

fn close_style(_: &Theme, status: button::Status) -> button::Style {
    let bg = match status {
        button::Status::Hovered | button::Status::Pressed => Color {
            r: 0.7,
            g: 0.2,
            b: 0.2,
            a: 1.0,
        },
        _ => Color {
            r: 0.25,
            g: 0.25,
            b: 0.25,
            a: 1.0,
        },
    };
    button::Style {
        background: Some(Background::Color(bg)),
        text_color: Color::WHITE,
        border: Border {
            radius: 4.0.into(),
            ..Default::default()
        },
        ..Default::default()
    }
}
