use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::widgets::Widget;

#[derive(Debug, Clone)]
pub struct Clear;

impl Widget for Clear {
  fn draw(&mut self, area: Rect, buf: &mut Buffer) {
    for x in area.left()..area.right() {
      for y in area.top()..area.bottom() {
        buf.get_mut(x, y).reset();
      }
    }
  }
}
