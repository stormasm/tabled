//! This module contains [`RawStyle`] structure, which is analogues to [`Style`] but not generic,
//! so sometimes it can be used more conviently.

// todo: StyleFromTable()
//       table.with(&mut StyleFromTable);
//       vs
//       Theme::from(table.get_config());
//           
// not sure what the best interface is
// IMHO 2

use core::iter::FromIterator;
use std::collections::HashMap;

use crate::{
    grid::{
        color::AnsiColor,
        config::{
            Border, Borders, ColoredConfig, CompactConfig, CompactMultilineConfig, HorizontalLine,
            VerticalLine,
        },
        records::Records,
    },
    settings::{style::Style, Color, TableOption},
};

use super::Colorization;

/// A raw style data, which can be produced safely from [`Style`].
///
/// It can be useful in order to not have a generics and be able to use it as a variable more conveniently.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Theme {
    border: TableBorders,
    lines: BorderLines,
    layout: Layout,
    colorization: Option<Colorization>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct TableBorders {
    chars: Borders<char>,
    colors: Borders<Color>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct BorderLines {
    horizontal1: Option<HorizontalLine<char>>,
    horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
    verticals: Option<HashMap<usize, VerticalLine<char>>>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Layout {
    orientation: Orientation,
    footer: bool,
    reverse_rows: bool,
    reverse_column: bool,
    move_header_on_borders: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Orientation {
    TopBottom,
    BottomTop,
    LeftRight,
    RightLeft,
}

impl Theme {
    pub const fn from_style<T, B, L, R, H, V, const HS: usize, const VS: usize>(
        style: Style<T, B, L, R, H, V, HS, VS>,
    ) -> Self {
        let chars = style.get_borders();
        let horizontals = style.get_horizontals();
        let horizontal1 = hlines_find(horizontals, 1);

        Self::_new(
            TableBorders::new(chars, Borders::empty()),
            BorderLines::new(horizontal1, None, None),
            Layout::new(Orientation::TopBottom, false, false, false, false),
            None,
        )
    }
}

impl Theme {
    /// Creates a new empty style.
    ///
    /// It's quite an analog of [`Style::empty`]
    pub const fn new() -> Self {
        Self::_new(
            TableBorders::new(Borders::empty(), Borders::empty()),
            BorderLines::new(None, None, None),
            Layout::new(Orientation::TopBottom, false, false, false, false),
            None,
        )
    }
}

macro_rules! func_set_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border character", " ", "<", $desc, ">", " ", ".")]
        pub fn $name(&mut self, c: char) {
            self.border.chars.$arg = Some(c);
        }
    };
}

macro_rules! func_remove_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border character", " ", "<", $desc, ">", " ", ".")]
        pub fn $name(&mut self) {
            self.border.chars.$arg = None;
        }
    };
}

macro_rules! func_get_chars {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Get a border character", " ", "<", $desc, ">", " ", ".")]
        pub const fn $name(&self) -> Option<char> {
            self.border.chars.$arg
        }
    };
}

macro_rules! func_set_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border color", " ", "<", $desc, ">", " ", ".")]
        pub fn $name(&mut self, color: Color) {
            self.border.colors.$arg = Some(color);
        }
    };
}

macro_rules! func_remove_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Remove a border color", " ", "<", $desc, ">", " ", ".")]
        pub fn $name(&mut self) {
            self.border.colors.$arg = None;
        }
    };
}

macro_rules! func_get_colors {
    ($name:ident, $arg:ident, $desc:expr) => {
        #[doc = concat!("Set a border color", " ", "<", $desc, ">", " ", ".")]
        pub fn $name(&self) -> Option<&Color> {
            self.border.colors.$arg.as_ref()
        }
    };
}

#[rustfmt::skip]
impl Theme {
    func_set_chars!(set_border_top,                      top,                        "top");
    func_set_chars!(set_border_bottom,                   bottom,                     "bottom");
    func_set_chars!(set_border_left,                     left,                       "left");
    func_set_chars!(set_border_right,                    right,                      "right");
    func_set_chars!(set_border_corner_top_left,          top_left,                   "top left corner");
    func_set_chars!(set_border_corner_top_right,         top_right,                  "top right corner");
    func_set_chars!(set_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_chars!(set_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_chars!(set_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_chars!(set_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_chars!(set_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_chars!(set_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_chars!(set_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_chars!(set_border_horizontal,               horizontal,                 "horizontal");
    func_set_chars!(set_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_chars!(get_border_top,                      top,                        "top");
    func_get_chars!(get_border_bottom,                   bottom,                     "bottom");
    func_get_chars!(get_border_left,                     left,                       "left");
    func_get_chars!(get_border_right,                    right,                      "right");
    func_get_chars!(get_border_corner_top_left,          top_left,                   "top left corner");
    func_get_chars!(get_border_corner_top_right,         top_right,                  "top right corner");
    func_get_chars!(get_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_chars!(get_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_chars!(get_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_chars!(get_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_chars!(get_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_chars!(get_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_chars!(get_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_chars!(get_border_horizontal,               horizontal,                 "horizontal");
    func_get_chars!(get_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_chars!(remove_border_top,                      top,                        "top");
    func_remove_chars!(remove_border_bottom,                   bottom,                     "bottom");
    func_remove_chars!(remove_border_left,                     left,                       "left");
    func_remove_chars!(remove_border_right,                    right,                      "right");
    func_remove_chars!(remove_border_corner_top_left,          top_left,                   "top left corner");
    func_remove_chars!(remove_border_corner_top_right,         top_right,                  "top right corner");
    func_remove_chars!(remove_border_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_chars!(remove_border_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_chars!(remove_border_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_chars!(remove_border_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_chars!(remove_border_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_chars!(remove_border_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_chars!(remove_border_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_chars!(remove_border_horizontal,               horizontal,                 "horizontal");
    func_remove_chars!(remove_border_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_set_colors!(set_border_color_top,                      top,                        "top");
    func_set_colors!(set_border_color_bottom,                   bottom,                     "bottom");
    func_set_colors!(set_border_color_left,                     left,                       "left");
    func_set_colors!(set_border_color_right,                    right,                      "right");
    func_set_colors!(set_border_color_corner_top_left,          top_left,                   "top left corner");
    func_set_colors!(set_border_color_corner_top_right,         top_right,                  "top right corner");
    func_set_colors!(set_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_set_colors!(set_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_set_colors!(set_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_set_colors!(set_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_set_colors!(set_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_set_colors!(set_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_set_colors!(set_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_set_colors!(set_border_color_horizontal,               horizontal,                 "horizontal");
    func_set_colors!(set_border_color_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_remove_colors!(remove_border_color_top,                      top,                        "top");
    func_remove_colors!(remove_border_color_bottom,                   bottom,                     "bottom");
    func_remove_colors!(remove_border_color_left,                     left,                       "left");
    func_remove_colors!(remove_border_color_right,                    right,                      "right");
    func_remove_colors!(remove_border_color_corner_top_left,          top_left,                   "top left corner");
    func_remove_colors!(remove_border_color_corner_top_right,         top_right,                  "top right corner");
    func_remove_colors!(remove_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_remove_colors!(remove_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_remove_colors!(remove_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_remove_colors!(remove_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_remove_colors!(remove_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_remove_colors!(remove_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_remove_colors!(remove_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_remove_colors!(remove_border_color_horizontal,               horizontal,                 "horizontal");
    func_remove_colors!(remove_border_color_vertical,                 vertical,                   "vertical");
}

#[rustfmt::skip]
impl Theme {
    func_get_colors!(get_border_color_top,                      top,                        "top");
    func_get_colors!(get_border_color_bottom,                   bottom,                     "bottom");
    func_get_colors!(get_border_color_left,                     left,                       "left");
    func_get_colors!(get_border_color_right,                    right,                      "right");
    func_get_colors!(get_border_color_corner_top_left,          top_left,                   "top left corner");
    func_get_colors!(get_border_color_corner_top_right,         top_right,                  "top right corner");
    func_get_colors!(get_border_color_corner_bottom_left,       bottom_left,                "bottom left corner");
    func_get_colors!(get_border_color_corner_bottom_right,      bottom_right,               "bottom right corner");
    func_get_colors!(get_border_color_intersection_top,         top_intersection,           "top intersection with a vertical line");
    func_get_colors!(get_border_color_intersection_bottom,      bottom_intersection,        "bottom intersection with a vertical line");
    func_get_colors!(get_border_color_intersection_left,        left_intersection,          "left intersection with a horizontal line");
    func_get_colors!(get_border_color_intersection_right,       right_intersection,         "right intersection with a horizontal line");
    func_get_colors!(get_border_color_intersection,             intersection,               "intersection of horizontal and vertical line");
    func_get_colors!(get_border_color_horizontal,               horizontal,                 "horizontal");
    func_get_colors!(get_border_color_vertical,                 vertical,                   "vertical");
}

impl Theme {
    /// Returns an outer border of the style.
    pub fn set_border_frame(&mut self, frame: Border<char>) {
        self.border.chars.top = frame.top;
        self.border.chars.bottom = frame.bottom;
        self.border.chars.left = frame.left;
        self.border.chars.right = frame.right;
        self.border.chars.top_left = frame.left_top_corner;
        self.border.chars.top_right = frame.right_top_corner;
        self.border.chars.bottom_left = frame.left_bottom_corner;
        self.border.chars.bottom_right = frame.right_bottom_corner;
    }

    /// Returns an outer border of the style.
    pub fn set_border_color_frame(&mut self, frame: Border<Color>) {
        self.border.colors.top = frame.top;
        self.border.colors.bottom = frame.bottom;
        self.border.colors.left = frame.left;
        self.border.colors.right = frame.right;
        self.border.colors.top_left = frame.left_top_corner;
        self.border.colors.top_right = frame.right_top_corner;
        self.border.colors.bottom_left = frame.left_bottom_corner;
        self.border.colors.bottom_right = frame.right_bottom_corner;
    }

    /// Set borders structure.
    pub fn set_border(&mut self, borders: Borders<char>) {
        self.border.chars = borders;
    }

    /// Set borders structure.
    pub fn set_border_color(&mut self, borders: Borders<Color>) {
        self.border.colors = borders;
    }

    /// Set an outer border.
    pub const fn get_border_frame(&self) -> Border<char> {
        Border {
            top: self.border.chars.top,
            bottom: self.border.chars.bottom,
            left: self.border.chars.left,
            right: self.border.chars.right,
            left_top_corner: self.border.chars.top_left,
            right_top_corner: self.border.chars.top_right,
            left_bottom_corner: self.border.chars.bottom_left,
            right_bottom_corner: self.border.chars.bottom_right,
        }
    }

    /// Set an outer border.
    pub const fn get_border_color_frame(&self) -> Border<&Color> {
        Border {
            top: self.border.colors.top.as_ref(),
            bottom: self.border.colors.bottom.as_ref(),
            left: self.border.colors.left.as_ref(),
            right: self.border.colors.right.as_ref(),
            left_top_corner: self.border.colors.top_left.as_ref(),
            right_top_corner: self.border.colors.top_right.as_ref(),
            left_bottom_corner: self.border.colors.bottom_left.as_ref(),
            right_bottom_corner: self.border.colors.bottom_right.as_ref(),
        }
    }

    /// Set horizontal border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{Table, settings::style::{Style, HorizontalLine}, settings::themes::Theme};
    ///
    /// let mut style = Theme::from(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(Style::extended()).into());
    ///
    /// style.set_lines_horizontal(lines);
    ///
    /// let data = (0..3).map(|i| ("Hello", i));
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         " ======= ===== \n",
    ///         "  &str    i32  \n",
    ///         "╠═══════╬═════╣\n",
    ///         "  Hello   0    \n",
    ///         "  Hello   1    \n",
    ///         "  Hello   2    \n",
    ///         " ======= ===== ",
    ///     ),
    /// )
    /// ```
    pub fn set_lines_horizontal(&mut self, lines: HashMap<usize, HorizontalLine<char>>) {
        self.lines.horizontals = Some(lines);
    }

    /// Set vertical border lines.
    ///
    /// # Example
    ///
    /// ```
    /// use std::collections::HashMap;
    /// use tabled::{
    ///     Table,
    ///     settings::style::{Style, HorizontalLine},
    ///     settings::themes::Theme,
    /// };
    ///
    ///
    /// let mut style = Theme::from_style(Style::re_structured_text());
    ///
    /// let mut lines = HashMap::new();
    /// lines.insert(1, HorizontalLine::inherit(Style::extended()).into());
    ///
    /// style.set_lines_vertical(lines);
    ///
    /// let data = (0..3).map(|i| ("Hello", i));
    /// let table = Table::new(data).with(style).to_string();
    ///
    /// assert_eq!(
    ///     table,
    ///     concat!(
    ///         "=======╠=====\n",
    ///         " &str  ═ i32 \n",
    ///         "======= =====\n",
    ///         " Hello ═ 0   \n",
    ///         " Hello ═ 1   \n",
    ///         " Hello ═ 2   \n",
    ///         "=======╣=====",
    ///     ),
    /// )
    /// ```
    pub fn set_lines_vertical(&mut self, lines: HashMap<usize, VerticalLine<char>>) {
        self.lines.verticals = Some(lines);
    }

    /// Insert a vertical line into specific column location.
    pub fn insert_line_vertical(&mut self, line: usize, vertical: VerticalLine<char>) {
        match &mut self.lines.verticals {
            Some(verticals) => {
                let _ = verticals.insert(line, vertical);
            }
            None => self.lines.verticals = Some(HashMap::from_iter([(line, vertical)])),
        }
    }

    /// Insert a horizontal line to a specific row location.
    pub fn insert_line_horizontal(&mut self, line: usize, horizontal: HorizontalLine<char>) {
        match &mut self.lines.horizontals {
            Some(horizontals) => {
                let _ = horizontals.insert(line, horizontal);
            }
            None => self.lines.horizontals = Some(HashMap::from_iter([(line, horizontal)])),
        }
    }

    /// Get a vertical line at the row if any set.
    pub fn get_line_vertical(&self, column: usize) -> Option<VerticalLine<char>> {
        self.lines
            .verticals
            .as_ref()
            .and_then(|lines| lines.get(&column).cloned())
    }

    /// Get a horizontal line at the row if any set.
    pub fn get_line_horizontal(&self, row: usize) -> Option<HorizontalLine<char>> {
        self.lines
            .horizontals
            .as_ref()
            .and_then(|list| list.get(&row).cloned())
    }
}

impl Theme {
    const fn _new(
        border: TableBorders,
        lines: BorderLines,
        layout: Layout,
        colorization: Option<Colorization>,
    ) -> Self {
        Self {
            border,
            lines,
            layout,
            colorization,
        }
    }
}

impl From<Borders<char>> for Theme {
    fn from(borders: Borders<char>) -> Self {
        Self::_new(
            TableBorders::new(borders, Borders::empty()),
            BorderLines::new(None, None, None),
            Layout::new(Orientation::TopBottom, false, false, false, false),
            None,
        )
    }
}

impl<R, D> TableOption<R, D, ColoredConfig> for Theme
where
    R: Records,
{
    fn change(self, _: &mut R, cfg: &mut ColoredConfig, _: &mut D) {
        cfg_clear_borders(cfg);
        cfg_set_custom_lines(cfg, self.lines);
        cfg_set_borders(cfg, self.border);
    }
}

impl<R, D> TableOption<R, D, CompactConfig> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactConfig, _: &mut D) {
        *cfg = cfg.set_borders(self.border.chars);
    }
}

impl<R, D> TableOption<R, D, CompactMultilineConfig> for Theme {
    fn change(self, _: &mut R, cfg: &mut CompactMultilineConfig, _: &mut D) {
        cfg.set_borders(self.border.chars);
    }
}

impl<T, B, L, R, H, V, const HSIZE: usize, const VSIZE: usize>
    From<Style<T, B, L, R, H, V, HSIZE, VSIZE>> for Theme
where
    T: Copy,
    B: Copy,
    L: Copy,
    R: Copy,
    H: Copy,
    V: Copy,
{
    fn from(style: Style<T, B, L, R, H, V, HSIZE, VSIZE>) -> Self {
        Self::from_style(style)
    }
}

impl From<ColoredConfig> for Theme {
    fn from(cfg: ColoredConfig) -> Self {
        let borders = *cfg.get_borders();
        let colors = borders_convert_ansi_color_to_color(cfg.get_color_borders().clone());
        let horizontals = cfg.get_horizontal_lines().into_iter().collect();
        let verticals = cfg.get_vertical_lines().into_iter().collect();

        Self::_new(
            TableBorders::new(borders, colors),
            BorderLines::new(None, Some(horizontals), Some(verticals)),
            Layout::new(Orientation::TopBottom, false, false, false, false),
            None,
        )
    }
}

fn borders_convert_ansi_color_to_color(b: Borders<AnsiColor<'static>>) -> Borders<Color> {
    Borders {
        top: b.top.map(Color::from),
        top_left: b.top_left.map(Color::from),
        top_right: b.top_right.map(Color::from),
        top_intersection: b.top_intersection.map(Color::from),
        bottom: b.bottom.map(Color::from),
        bottom_left: b.bottom_left.map(Color::from),
        bottom_right: b.bottom_right.map(Color::from),
        bottom_intersection: b.bottom_intersection.map(Color::from),
        horizontal: b.horizontal.map(Color::from),
        vertical: b.vertical.map(Color::from),
        intersection: b.intersection.map(Color::from),
        left: b.left.map(Color::from),
        left_intersection: b.left_intersection.map(Color::from),
        right: b.right.map(Color::from),
        right_intersection: b.right_intersection.map(Color::from),
    }
}

fn borders_convert_color_to_ansi_color(b: Borders<Color>) -> Borders<AnsiColor<'static>> {
    Borders {
        top: b.top.map(AnsiColor::from),
        top_left: b.top_left.map(AnsiColor::from),
        top_right: b.top_right.map(AnsiColor::from),
        top_intersection: b.top_intersection.map(AnsiColor::from),
        bottom: b.bottom.map(AnsiColor::from),
        bottom_left: b.bottom_left.map(AnsiColor::from),
        bottom_right: b.bottom_right.map(AnsiColor::from),
        bottom_intersection: b.bottom_intersection.map(AnsiColor::from),
        horizontal: b.horizontal.map(AnsiColor::from),
        vertical: b.vertical.map(AnsiColor::from),
        intersection: b.intersection.map(AnsiColor::from),
        left: b.left.map(AnsiColor::from),
        left_intersection: b.left_intersection.map(AnsiColor::from),
        right: b.right.map(AnsiColor::from),
        right_intersection: b.right_intersection.map(AnsiColor::from),
    }
}

fn cfg_clear_borders(cfg: &mut ColoredConfig) {
    cfg.remove_borders();
    cfg.remove_borders_colors();
    cfg.remove_vertical_chars();
    cfg.remove_horizontal_chars();
    cfg.remove_color_line_horizontal();
    cfg.remove_color_line_vertical();
}

fn cfg_set_borders(cfg: &mut ColoredConfig, border: TableBorders) {
    cfg.set_borders(border.chars);

    if !border.colors.is_empty() {
        cfg.set_borders_color(borders_convert_color_to_ansi_color(border.colors));
    }
}

fn cfg_set_custom_lines(cfg: &mut ColoredConfig, lines: BorderLines) {
    if let Some(line) = lines.horizontal1 {
        cfg.insert_horizontal_line(1, line);
    }

    if let Some(lines) = lines.horizontals {
        for (row, line) in lines {
            cfg.insert_horizontal_line(row, line);
        }
    }

    if let Some(lines) = lines.verticals {
        for (col, line) in lines {
            cfg.insert_vertical_line(col, line);
        }
    }
}

const fn hlines_find<const N: usize>(
    lines: [(usize, HorizontalLine<char>); N],
    search: usize,
) -> Option<HorizontalLine<char>> {
    let mut line = None;

    let mut i = 0;
    while i < lines.len() {
        let (num, hline) = lines[i];
        if num == search {
            line = Some(hline);
        }

        i += 1;
    }

    line
}

impl TableBorders {
    const fn new(chars: Borders<char>, colors: Borders<Color>) -> Self {
        Self { chars, colors }
    }
}

impl BorderLines {
    const fn new(
        horizontal1: Option<HorizontalLine<char>>,
        horizontals: Option<HashMap<usize, HorizontalLine<char>>>,
        verticals: Option<HashMap<usize, VerticalLine<char>>>,
    ) -> Self {
        Self {
            horizontal1,
            horizontals,
            verticals,
        }
    }
}

impl Layout {
    const fn new(
        orientation: Orientation,
        footer: bool,
        reverse_rows: bool,
        reverse_column: bool,
        move_header_on_borders: bool,
    ) -> Self {
        Self {
            orientation,
            footer,
            reverse_rows,
            reverse_column,
            move_header_on_borders,
        }
    }
}