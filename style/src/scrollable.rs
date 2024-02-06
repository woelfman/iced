//! Change the appearance of a scrollable.
use crate::core::{Background, Border, Color};

/// The appearance of the scrollbar of a scrollable.
#[derive(Debug, Clone, Copy)]
pub struct Scrollbar {
    /// The [`Background`] of a scrollbar.
    pub background: Option<Background>,
    /// The [`Border`] of a scrollbar.
    pub border: Border,
    /// The appearance of the [`Scroller`] of a scrollbar.
    pub scroller: Scroller,
}

/// The appearance of the scroller of a scrollable.
#[derive(Debug, Clone, Copy)]
pub struct Scroller {
    /// The [`Color`] of the scroller.
    pub color: Color,
    /// The [`Border`] of the scroller.
    pub border: Border,
}

/// The appearance of a scrolable.
#[derive(Debug, Clone, Copy)]
pub struct Appearance {
    /// The [`Background`] of a scrollable.
    pub background: Option<Background>,
    /// The [`Background`] of the gap between a horizontal and vertical scrollbar.
    pub gap: Option<Background>,
}

/// A set of rules that dictate the style of a scrollable.
pub trait StyleSheet {
    /// The supported style of the [`StyleSheet`].
    type Style: Default;

    /// Produces the style of the scrollable container.
    fn appearance(&self, style: &Self::Style) -> Appearance;

    /// Produces the style of an active scrollbar.
    fn active(&self, style: &Self::Style) -> Scrollbar;

    /// Produces the style of a scrollbar when the scrollable is being hovered.
    fn hovered(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> Scrollbar;

    /// Produces the style of a scrollbar that is being dragged.
    fn dragging(&self, style: &Self::Style) -> Scrollbar {
        self.hovered(style, true)
    }

    /// Produces the style of an active horizontal scrollbar.
    fn active_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.active(style)
    }

    /// Produces the style of a horizontal scrollbar when the scrollable is being hovered.
    fn hovered_horizontal(
        &self,
        style: &Self::Style,
        is_mouse_over_scrollbar: bool,
    ) -> Scrollbar {
        self.hovered(style, is_mouse_over_scrollbar)
    }

    /// Produces the style of a horizontal scrollbar that is being dragged.
    fn dragging_horizontal(&self, style: &Self::Style) -> Scrollbar {
        self.hovered_horizontal(style, true)
    }
}
