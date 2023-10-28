use iced::widget::{checkbox, container, pick_list, row, slider};
use iced::{
    alignment::{Horizontal, Vertical},
    Color, Element, Length, Sandbox, Settings,
};
use iced_aw::Grid;

struct App {
    horizontal_alignment: Horizontal,
    vertical_alignment: Vertical,
    column_spacing: f32,
    row_spacing: f32,
    fill_width: bool,
    fill_height: bool,
    padding: f32,
    debug_layout: bool,
}

#[derive(Debug, Clone)]
enum Message {
    HorizontalAlignment(Horizontal),
    VerticalAlignment(Vertical),
    ColumnSpacing(f32),
    RowSpacing(f32),
    FillWidth(bool),
    FillHeight(bool),
    Padding(f32),
    DebugToggled(bool),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self {
            horizontal_alignment: Horizontal::Left,
            vertical_alignment: Vertical::Center,
            column_spacing: 5.0,
            row_spacing: 5.0,
            fill_width: false,
            fill_height: false,
            padding: 0.0,
            debug_layout: false,
        }
    }

    fn title(&self) -> String {
        "Iced Grid widget example".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Message::HorizontalAlignment(align) => self.horizontal_alignment = align,
            Message::VerticalAlignment(align) => self.vertical_alignment = align,
            Message::ColumnSpacing(spacing) => self.column_spacing = spacing,
            Message::RowSpacing(spacing) => self.row_spacing = spacing,
            Message::FillWidth(fill) => self.fill_width = fill,
            Message::FillHeight(fill) => self.fill_height = fill,
            Message::Padding(value) => self.padding = value,
            Message::DebugToggled(enabled) => self.debug_layout = enabled,
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let horizontal_align_pick = pick_list(
            HORIZONTAL_ALIGNMENTS
                .iter()
                .map(horizontal_align_to_string)
                .collect::<Vec<_>>(),
            Some(horizontal_align_to_string(&self.horizontal_alignment)),
            |selected| Message::HorizontalAlignment(string_to_horizontal_align(&selected)),
        );

        let vertical_align_pick = pick_list(
            VERTICAL_ALIGNMENTS
                .iter()
                .map(vertical_alignment_to_string)
                .collect::<Vec<_>>(),
            Some(vertical_alignment_to_string(&self.vertical_alignment)),
            |selected| Message::VerticalAlignment(string_to_vertical_align(&selected)),
        );

        let row_spacing_slider =
            slider(0.0..=100.0, self.row_spacing, Message::RowSpacing).width(Length::Fill);
        let col_spacing_slider =
            slider(0.0..=100.0, self.column_spacing, Message::ColumnSpacing).width(Length::Fill);

        let debug_mode_check = checkbox("", self.debug_layout, Message::DebugToggled);

        let fill_checkboxes = row![
            checkbox("Width", self.fill_width, Message::FillWidth),
            checkbox("Height", self.fill_height, Message::FillHeight)
        ]
        .spacing(10);

        let padding_slider =
            slider(0.0..=100.0, self.padding, Message::Padding).width(Length::Fixed(400.0));

        // let mut grid = grid!(
        //     grid_row!("Horizontal alignment", horizontal_align_pick,),
        //     grid_row!("Vertical alignment", vertical_align_pick),
        //     grid_row!("Row spacing", row_spacing_slider),
        //     grid_row!("Column spacing", col_spacing_slider),
        //     grid_row!("Fill space", fill_checkboxes),
        //     grid_row!("Padding", padding_slider),
        //     grid_row!("Debug mode", debug_mode_check)
        // )
        // .horizontal_alignment(self.horizontal_alignment)
        // .vertical_alignment(self.vertical_alignment)
        // .row_spacing(self.row_spacing)
        // .column_spacing(self.column_spacing)
        // .padding(Padding::new(self.padding));

        // if self.fill_width {
        //     grid = grid.width(Length::Fill);
        // }
        // if self.fill_height {
        //     grid = grid.height(Length::Fill);
        // }
        let grid = Grid::new()
            .push("Horizontal alignment")
            .push(horizontal_align_pick)
            .end_row()
            .push("Vertical alignment")
            .push(vertical_align_pick)
            .end_row()
            .push("Row spacing")
            .push(row_spacing_slider)
            .end_row()
            .push("Columns spacing")
            .push(col_spacing_slider)
            .end_row()
            .push("Fill space")
            .push(fill_checkboxes)
            .end_row()
            .push("Padding")
            .push(padding_slider)
            .end_row()
            .push("Debug layout")
            .push(debug_mode_check)
            .end_row();

        let mut contents = Element::from(grid);
        if self.debug_layout {
            contents = contents.explain(Color::BLACK);
        }
        container(contents)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

const HORIZONTAL_ALIGNMENTS: [Horizontal; 3] =
    [Horizontal::Left, Horizontal::Center, Horizontal::Right];

const VERTICAL_ALIGNMENTS: [Vertical; 3] = [Vertical::Top, Vertical::Center, Vertical::Bottom];

fn horizontal_align_to_string(alignment: &Horizontal) -> String {
    match alignment {
        Horizontal::Left => "Left",
        Horizontal::Center => "Center",
        Horizontal::Right => "Right",
    }
    .to_string()
}

fn string_to_horizontal_align(input: &str) -> Horizontal {
    match input {
        "Left" => Horizontal::Left,
        "Center" => Horizontal::Center,
        "Right" => Horizontal::Right,
        _ => panic!(),
    }
}

fn vertical_alignment_to_string(alignment: &Vertical) -> String {
    match alignment {
        Vertical::Top => "Top",
        Vertical::Center => "Center",
        Vertical::Bottom => "Bottom",
    }
    .to_string()
}

fn string_to_vertical_align(input: &str) -> Vertical {
    match input {
        "Top" => Vertical::Top,
        "Center" => Vertical::Center,
        "Bottom" => Vertical::Bottom,
        _ => panic!(),
    }
}

fn main() -> iced::Result {
    App::run(Settings::default())
}
