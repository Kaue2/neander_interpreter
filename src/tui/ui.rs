use std::fmt::format;

use ratatui::{
  Frame, 
  layout::{Constraint, Direction, Layout}, 
  style::{Color, Style, Stylize}, 
  symbols::border, 
  text::{Line, Span}, 
  widgets::{Block, Borders, Paragraph, Scrollbar}
};

use tui_big_text::{BigText, PixelSize};

use crate::{App, Interpreter};

pub fn ui(frame: &mut Frame, interpreter: &Interpreter, app: &mut App) {
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

  let mut program_text = Vec::<Line>::new();
  let relevant_data = interpreter.memory.get(4..).unwrap_or(&[]);
  let converted_data = interpreter.convert_data();
  for (i, line) in relevant_data.chunks(2).enumerate() {
    program_text.push(Line::from(Span::styled(
      format!("{:03} - {:02X} - {:03}", i, line[0], converted_data[i]), 
      Style::new().bold().fg(Color::White)
    )));
  };

  let program_paragraph = Paragraph::new(program_text)
    .gray()
    .centered()
    .block(program_title_block)
    .scroll((app.program_scroll as u16, 0));

  let mut data_text = Vec::<Line>::new();
  for (i, line) in relevant_data.chunks(2).enumerate() {
    data_text.push(Line::from(Span::styled(
      format!("{:03} - {:02X}", i, line[0]),
      Style::new().bold().fg(Color::Green)
    )));
  }

  let data_paragraph = Paragraph::new(data_text)
    .gray()
    .centered()
    .block(data_title_block)
    .scroll((app.data_scroll as u16, 0));

  let acc = format!("{:03}", interpreter.ac);
  let text_ac = format!("ACC: {}", acc);

  let acc_art = BigText::builder()
    .pixel_size(PixelSize::Full)
    .style(Style::new().bold().fg(Color::Cyan))
    .lines(vec![Line::from(text_ac)])
    .build();
  
  frame.render_widget(program_paragraph, chunks[0]);
  frame.render_widget(main_title_block, chunks[1]);
  frame.render_widget(data_paragraph, chunks[2]);
  frame.render_widget(acc_art, chunks[1]);

  frame.render_stateful_widget(
    Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
      .begin_symbol(Some("↑"))
      .end_symbol(Some("↓")),
    chunks[0], 
    &mut app.program_scroll_state
  );
  frame.render_stateful_widget(
    Scrollbar::new(ratatui::widgets::ScrollbarOrientation::VerticalRight)
      .begin_symbol(Some("↑"))
      .end_symbol(Some("↓")),
    chunks[2],
    &mut app.data_scroll_state
  );
}