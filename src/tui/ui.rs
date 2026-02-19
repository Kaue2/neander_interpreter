use ratatui::{
  Frame, 
  layout::{Constraint, Direction, Layout}, 
  style::{Color, Style}, 
  symbols::border, 
  text::{Line}, 
  widgets::{Block, Borders}
};

use crate::Interpreter;

pub fn ui(frame: &mut Frame, interpreter: &Interpreter) {
  let chunks = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(20),
      Constraint::Fill(1),
      Constraint::Percentage(20),
    ])
    .split(frame.area());

  let program_title = Line::from(" Program ");
  let program_title_block = Block::default()
      .title(program_title.centered().style(Color::White))
      .style(Style::new().bold().fg(Color::Blue))
      .borders(Borders::ALL)
      .border_set(border::ROUNDED);

  let main_title = Line::from(" Neander Interpreter ");
  let main_title_block = Block::default()
      .title(main_title.centered().style(Color::White))
      .style(Style::new().bold().fg(Color::Blue))
      .borders(Borders::ALL)
      .border_set(border::ROUNDED);

  let data_title = Line::from(" Data ");
  let data_title_block = Block::default()
      .title(data_title.centered().style(Color::White))
      .style(Style::new().bold().fg(Color::Blue))
      .borders(Borders::ALL)
      .border_set(border::ROUNDED);

  frame.render_widget(program_title_block, chunks[0]);
  frame.render_widget(main_title_block, chunks[1]);
  frame.render_widget(data_title_block, chunks[2]);
}