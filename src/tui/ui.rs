use ratatui::{
  Frame, 
  layout::{Constraint, Direction, Layout}, 
  style::{Color, Style, Stylize}, 
  symbols::border, 
  text::{Line, Span}, 
  widgets::{Block, Borders, Paragraph, Scrollbar, Padding}
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
      .padding(Padding::new(1, 1, 1, 1))
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

  let acc_block = Block::default()
    .title(Line::from(" Accumulator ").centered().fg(Color::White))
    .padding(Padding::new(1, 1, 1, 1));
  let acc = format!("{:03}", interpreter.ac);
  let text_acc = format!("ACC: {}", acc);

  let acc_art = BigText::builder()
    .pixel_size(PixelSize::HalfHeight)
    .style(Style::new().bold().fg(Color::Green))
    .lines(vec![Line::from(text_acc)])
    .centered()
    .build();

  let pc_block = Block::default()
    .title(Line::from(" Program Counter ").centered().centered().fg(Color::White))
    .padding(Padding::new(1, 1, 1, 1));
  let pc = format!("{:03}", interpreter.pc.address());
  let text_pc = format!("PC: {}", pc);
  
  let pc_art = BigText::builder()
    .pixel_size(PixelSize::HalfHeight)
    .style(Style::new().bold().fg(Color::Green))
    .lines(vec![Line::from(text_pc)])
    .centered()
    .build();

  let zero_flag_block = Block::default()
    .title(Line::from(" Zero ").centered().centered().fg(Color::White))
    .padding(Padding::new(1, 1, 1, 1));
  let zero = if interpreter.zero {
    format!("True")
  } else {
    format!("False")
  };

  let zero_art = BigText::builder()
    .pixel_size(PixelSize::HalfHeight)
    .style(Style::new().bold().fg(Color::Green))
    .lines(vec![Line::from(zero)])
    .centered()
    .build();

  let negative_flag_block = Block::default()
    .title(Line::from(" Negative ").centered().centered().fg(Color::White))
    .padding(Padding::new(1, 1, 1, 1));
  let negative = if interpreter.negative {
    format!("True")
  } else {
    format!("False")
  };

  let negative_art = BigText::builder()
    .pixel_size(PixelSize::HalfHeight)
    .style(Style::new().bold().fg(Color::Green))
    .lines(vec![Line::from(negative)])
    .centered()
    .build();
  
  let params_inner_area = main_title_block.inner(chunks[1]);

  let main_split = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ])
    .split(params_inner_area);
  
  let program_vars = Layout::default()
    .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ])
    .split(main_split[0]);

  let program_flags = Layout::default()
      .direction(Direction::Horizontal)
    .constraints([
      Constraint::Percentage(50),
      Constraint::Percentage(50),
    ])
    .split(main_split[1]);

  let acc_inner_area = acc_block.inner(program_vars[0]);
  let pc_inner_area = pc_block.inner(program_vars[1]);
  let zero_inner_area = zero_flag_block.inner(program_flags[0]);
  let negative_inner_area = zero_flag_block.inner(program_flags[1]);

  frame.render_widget(program_paragraph, chunks[0]);
  frame.render_widget(main_title_block, chunks[1]);
  frame.render_widget(data_paragraph, chunks[2]);
  frame.render_widget(acc_block, program_vars[0]);
  frame.render_widget(acc_art, acc_inner_area);
  frame.render_widget(pc_block, program_vars[1]);
  frame.render_widget(pc_art, pc_inner_area);
  frame.render_widget(zero_flag_block, program_flags[0]);
  frame.render_widget(zero_art, zero_inner_area);
  frame.render_widget(negative_flag_block, program_flags[1]);
  frame.render_widget(negative_art, negative_inner_area);

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