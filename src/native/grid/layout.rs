use iced_widget::core::{
    alignment::{Horizontal, Vertical},
    layout::{Limits, Node},
    Element, Length, Padding, Pixels, Point, Size,
};
use taffy::{
    error::TaffyError,
    geometry::Size as TaffySize,
    prelude::{Line, Rect},
    style::{AvailableSpace, Dimension, Display, GridPlacement, LengthPercentage, Style},
    Taffy,
};

use super::types::Position;

#[allow(clippy::too_many_arguments)]
pub(super) fn layout<Message, Renderer>(
    renderer: &Renderer,
    limits: &Limits,
    children: &[Element<'_, Message, Renderer>],
    positions: &[Position],
    column_spacing: Pixels,
    row_spacing: Pixels,
    padding: Padding,
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    width: Length,
    height: Length,
    column_lengths: &[Length],
    row_lengths: &[Length],
) -> Result<Node, TaffyError>
where
    Renderer: iced_widget::core::Renderer,
{
    let max_size = limits.max();

    let mut leafs = Vec::with_capacity(children.len());
    let mut nodes = Vec::with_capacity(children.len());

    let mut taffy = Taffy::with_capacity(children.len() + 1);

    for (element, position) in children.iter().zip(positions) {
        let widget = element.as_widget();
        let node = widget.layout(renderer, limits);
        let size = node.size();

        nodes.push(node);

        let width = match widget.width() {
            Length::Fill | Length::FillPortion(_) => Dimension::Auto,
            Length::Shrink | Length::Fixed(_) => Dimension::Points(size.width),
        };
        let height = match widget.height() {
            Length::Fill | Length::FillPortion(_) => Dimension::Auto,
            Length::Shrink | Length::Fixed(_) => Dimension::Points(size.height),
        };

        let leaf = taffy.new_leaf(Style {
            // display: (),
            // position: (),
            // inset: (),
            size: TaffySize { width, height },
            // min_size: (),
            // max_size: (),
            // aspect_ratio: (),
            // margin: (),
            // padding: (),
            // border: (),
            // align_items: (),
            // align_self: (),
            // justify_items: (),
            // justify_self: (),
            // align_content: (),
            // justify_content: (),
            // gap: (),
            // flex_direction: (),
            // flex_wrap: (),
            // flex_basis: (),
            // flex_grow: (),
            // flex_shrink: (),
            // grid_template_rows: (),
            // grid_template_columns: (),
            // grid_auto_rows: (),
            // grid_auto_columns: (),
            // grid_auto_flow: (),
            grid_row: Line {
                start: GridPlacement::Line((position.row as i16).into()),
                end: GridPlacement::Line(((position.row + position.height) as i16).into()),
            },
            grid_column: Line {
                start: GridPlacement::Line((position.column as i16).into()),
                end: GridPlacement::Line(((position.column + position.width) as i16).into()),
            },
            ..Default::default()
        })?;

        leafs.push(leaf);
    }

    let root = taffy.new_with_children(
        Style {
            display: Display::Grid,
            // position: (),
            // inset: (),
            // size: (),
            // min_size: (),
            // max_size: (),
            // aspect_ratio: (),
            // margin: (),
            padding: Rect {
                left: length(padding.left),
                right: length(padding.right),
                top: length(padding.top),
                bottom: length(padding.bottom),
            },
            // border: (),
            // align_items: (),
            // align_self: (),
            // justify_items: (),
            // justify_self: (),
            // align_content: (),
            // justify_content: (),
            gap: TaffySize {
                width: length(column_spacing.0),
                height: length(row_spacing.0),
            },
            // flex_direction: (),
            // flex_wrap: (),
            // flex_basis: (),
            // flex_grow: (),
            // flex_shrink: (),
            // grid_template_rows: (),
            // grid_template_columns: (),
            // grid_auto_rows: (),
            // grid_auto_columns: (),
            // grid_auto_flow: (),
            // grid_row: (),
            // grid_column: (),
            ..Default::default()
        },
        &leafs,
    )?;

    taffy.compute_layout(
        root,
        TaffySize {
            width: AvailableSpace::Definite(max_size.width),
            height: AvailableSpace::Definite(max_size.height),
        },
    )?;

    let grid_layout = taffy.layout(root)?;

    for ((leaf, element), node) in leafs.into_iter().zip(children).zip(nodes.iter_mut()) {
        let leaf_layout = taffy.layout(leaf)?;
        let widget = element.as_widget();
        match widget.width() {
            Length::Fill | Length::FillPortion(_) => {
                *node = widget.layout(renderer, &limits.width(leaf_layout.size.width));
            }
            _ => (),
        }

        node.move_to(Point {
            x: leaf_layout.location.x,
            y: leaf_layout.location.y,
        });
    }

    let grid_size = Size {
        width: grid_layout.size.width,
        height: grid_layout.size.height,
    };

    Ok(Node::with_children(grid_size.pad(padding), nodes))
}

fn length(value: f32) -> LengthPercentage {
    LengthPercentage::Points(value)
}
