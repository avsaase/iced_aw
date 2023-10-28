use iced_widget::core::{
    alignment::{Horizontal, Vertical},
    Element, Length, Padding, Pixels,
};

/// A container that distributes its contents in a grid of rows and columns.
///
/// The number of columns is determined by the row with the most elements.
#[allow(missing_debug_implementations)]
pub struct Grid<'a, Message, Renderer = crate::Renderer> {
    pub(super) children: Vec<Element<'a, Message, Renderer>>,
    pub(super) positions: Vec<Position>,
    pub(super) horizontal_alignment: Horizontal,
    pub(super) vertical_alignment: Vertical,
    pub(super) column_spacing: Pixels,
    pub(super) row_spacing: Pixels,
    pub(super) padding: Padding,
    pub(super) width: Length,
    pub(super) height: Length,
    pub(super) column_widths: Vec<Length>,
    pub(super) row_heights: Vec<Length>,
    pub(super) column: u16,
    pub(super) row: u16,
}

impl<'a, Message, Renderer> Default for Grid<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    fn default() -> Self {
        Self {
            children: Vec::new(),
            positions: Vec::new(),
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            column_spacing: 1.0.into(),
            row_spacing: 1.0.into(),
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            column_widths: vec![Length::Fill],
            row_heights: vec![Length::Fill],
            column: 0,
            row: 0,
        }
    }
}

impl<'a, Message, Renderer> Grid<'a, Message, Renderer>
where
    Renderer: iced_widget::core::Renderer,
{
    /// Creates a new [`Grid`].
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a widget to the [`Grid`]. The widget is added as a new column in the current row.
    #[must_use]
    pub fn push(mut self, widget: impl Into<Element<'a, Message, Renderer>>) -> Self {
        self.children.push(widget.into());
        self.positions.push(Position {
            column: self.column,
            row: self.row,
            width: 1,
            height: 1,
        });
        self.column += 1;
        self
    }

    /// Adds a widget to the [`Grid`] at the given [`Position`].
    #[must_use]
    pub fn push_at_position(
        mut self,
        widget: impl Into<Element<'a, Message, Renderer>>,
        position: impl Into<Position>,
    ) -> Self {
        let position: Position = position.into();
        self.children.push(widget.into());
        self.positions.push(position);
        self.column = position.column + position.width;
        self.row = position.row + position.height;
        self
    }

    /// End the current row. Subsequent widget are added to a new row.
    #[must_use]
    pub fn end_row(mut self) -> Self {
        self.column = 0;
        self.row += 1;
        self
    }

    /// Sets the horizontal alignment of the widgets within their cells. Default:
    /// [`Horizontal::Left`]
    #[must_use]
    pub fn horizontal_alignment(mut self, align: Horizontal) -> Self {
        self.horizontal_alignment = align;
        self
    }

    /// Sets the vertical alignment of the widgets within their cells. Default:
    /// [`Vertical::Center`]
    #[must_use]
    pub fn vertical_alignment(mut self, align: Vertical) -> Self {
        self.vertical_alignment = align;
        self
    }

    /// Sets the spacing between rows and columns. To set row and column spacing separately, use
    /// [`Self::column_spacing()`] and [`Self::row_spacing()`].
    #[must_use]
    pub fn spacing(mut self, spacing: f32) -> Self {
        let spacing: Pixels = spacing.into();
        self.row_spacing = spacing;
        self.column_spacing = spacing;
        self
    }

    /// Sets the spacing between columns.
    #[must_use]
    pub fn column_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.column_spacing = spacing.into();
        self
    }

    /// Sets the spacing between rows.
    #[must_use]
    pub fn row_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.row_spacing = spacing.into();
        self
    }

    /// Sets the padding around the grid.
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the grid width.
    #[must_use]
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the grid height.
    #[must_use]
    pub fn height(mut self, height: impl Into<Length>) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the column width.
    ///
    /// The same setting will be used for all columns. To set separate values for each column, use
    /// [`Self::column_widths()`]. Columns are never smaller than the space needed to fit their
    /// contents.
    #[must_use]
    pub fn column_width(mut self, width: impl Into<Length>) -> Self {
        self.column_widths = vec![width.into()];
        self
    }

    /// Sets the row height.
    ///
    /// The same setting will be used for all rows. To set separate values for each row, use
    /// [`Self::row_heights()`]. Rows are never smaller than the space needed to fit their
    /// contents.
    #[must_use]
    pub fn row_height(mut self, height: impl Into<Length>) -> Self {
        self.row_heights = vec![height.into()];
        self
    }

    /// Sets a separate width for each column.
    ///
    /// Columns are never smaller than the space needed to fit their contents. When supplying fewer
    /// values than the number of columns, values are are repeated using
    /// [`std::iter::Iterator::cycle()`].
    #[must_use]
    pub fn column_widths(mut self, widths: &[Length]) -> Self {
        self.column_widths = widths.into();
        self
    }

    /// Sets a separate height for each row.
    ///
    /// Rows are never smaller than the space needed to fit their contents. When supplying fewer
    /// values than the number of rows, values are are repeated using
    /// [`std::iter::Iterator::cycle()`].
    #[must_use]
    pub fn row_heights(mut self, heights: &[Length]) -> Self {
        self.row_heights = heights.into();
        self
    }

    // pub(super) fn elements_iter(&self) -> impl Iterator<Item = &Element<'a, Message, Renderer>> {
    //     self.rows.iter().flat_map(|row| row.elements.iter())
    // }

    // pub(super) fn elements_iter_mut(
    //     &mut self,
    // ) -> impl Iterator<Item = &mut Element<'a, Message, Renderer>> {
    //     self.rows.iter_mut().flat_map(|row| row.elements.iter_mut())
    // }

    // pub(super) fn column_count(&self) -> usize {
    //     self.rows
    //         .iter()
    //         .map(|row| row.elements.len())
    //         .max()
    //         .unwrap_or(0)
    // }

    // pub(super) fn row_count(&self) -> usize {
    //     self.rows.len()
    // }

    // pub(super) fn element_count(&self) -> usize {
    //     self.rows.iter().map(|row| row.elements.len()).sum()
    // }
}

/// The position and size of a widget in the [`Grid`].
#[derive(Debug, Clone, Copy)]
pub struct Position {
    pub(super) column: u16,
    pub(super) row: u16,
    pub(super) width: u16,
    pub(super) height: u16,
}

impl Position {
    /// Creates a new [`Position`]
    pub fn new(column: u16, row: u16, width: u16, height: u16) -> Self {
        Self {
            column,
            row,
            width,
            height,
        }
    }
}

impl From<(u16, u16)> for Position {
    fn from(value: (u16, u16)) -> Self {
        Self {
            column: value.0,
            row: value.1,
            width: 1,
            height: 1,
        }
    }
}

impl From<(u16, u16, u16, u16)> for Position {
    fn from(value: (u16, u16, u16, u16)) -> Self {
        Self {
            column: value.0,
            row: value.1,
            width: value.2,
            height: value.3,
        }
    }
}
