use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use std::io::{self, stdout, Stdout};

use crate::app::App;

#[derive(Debug)]
pub struct Tui {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    app: App,
}

impl Tui {
    pub fn init(app: App) -> Self {
        let _ = execute!(stdout(), EnterAlternateScreen);
        let _ = enable_raw_mode();
        let stdout = io::stdout();

        let backend = CrosstermBackend::new(stdout);
        let terminal = Terminal::new(backend).unwrap();

        Tui { terminal, app }
    }

    pub fn render(&mut self) {
        let active_buttons = self.active_buttons();
        let _ = self.terminal.clear();
        let _ = self.terminal.draw(|f| {
            let main_chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(10),
                        Constraint::Percentage(80),
                        Constraint::Percentage(10),
                    ]
                    .as_ref(),
                )
                .split(f.area());

            let top_instruction = p_center("Quit: q");
            f.render_widget(top_instruction, main_chunks[0]);

            let calculator_block = Block::default().borders(Borders::ALL).title("calculator-tui");
            f.render_widget(calculator_block, main_chunks[1]);

            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Percentage(20),
                        Constraint::Percentage(10),
                        Constraint::Percentage(70),
                    ]
                    .as_ref(),
                )
                .split(main_chunks[1]);

            let binding = self.app.get_display();
            let result = binding.as_str();
            let input = input(result);

            f.render_widget(input, chunks[0]);

            let layout_vertical = layout_buttons_vertical().split(chunks[2]);
            let row0 = layout_button_horizontal().split(layout_vertical[0]);
            let row1 = layout_button_horizontal().split(layout_vertical[1]);
            let row2 = layout_button_horizontal().split(layout_vertical[2]);
            let row3 = layout_button_horizontal().split(layout_vertical[3]);
            let row4 = layout_button_horizontal().split(layout_vertical[4]);

            // row 0 (vide, vide, vide, AC)
            let button_reset = button_secondary("[AC]", active_buttons[0][3]);
            f.render_widget(empty(), row0[0]);
            f.render_widget(empty(), row0[1]);
            f.render_widget(empty(), row0[2]);
            f.render_widget(button_reset, row0[3]);

            // row 1 (7, 8, 9, /)
            let button7 = button_primary("[7]", active_buttons[1][0]);
            let button8 = button_primary("[8]", active_buttons[1][1]);
            let button9 = button_primary("[9]", active_buttons[1][2]);
            let button_div = button_secondary("[/]", active_buttons[1][3]);

            f.render_widget(button7, row1[0]);
            f.render_widget(button8, row1[1]);
            f.render_widget(button9, row1[2]);
            f.render_widget(button_div, row1[3]);

            // row 2 (4, 5, 6, *)
            let button4 = button_primary("[4]", active_buttons[2][0]);
            let button5 = button_primary("[5]", active_buttons[2][1]);
            let button6 = button_primary("[6]", active_buttons[2][2]);
            let button_mul = button_secondary("[*]", active_buttons[2][3]);

            f.render_widget(button4, row2[0]);
            f.render_widget(button5, row2[1]);
            f.render_widget(button6, row2[2]);
            f.render_widget(button_mul, row2[3]);

            // row 3 (1, 2, 3, -)
            let button1 = button_primary("[1]", active_buttons[3][0]);
            let button2 = button_primary("[2]", active_buttons[3][1]);
            let button3 = button_primary("[3]", active_buttons[3][2]);
            let button_sub = button_secondary("[-]", active_buttons[3][3]);

            f.render_widget(button1, row3[0]);
            f.render_widget(button2, row3[1]);
            f.render_widget(button3, row3[2]);
            f.render_widget(button_sub, row3[3]);

            // row 4 (0, vide, =, +)
            let button0 = button_primary("[0]", active_buttons[4][0]);
            let button_eq = button_secondary("[=]", active_buttons[4][2]);
            let button_add = button_secondary("[+]", active_buttons[4][3]);

            f.render_widget(button0, row4[0]);
            f.render_widget(button_primary("", active_buttons[4][1]), row4[1]);
            f.render_widget(button_eq, row4[2]);
            f.render_widget(button_add, row4[3]);

            let bottom_instruction = p_center("Use arrow keys (left, right, up, down) to move and press Enter to select");
            //Use the arrow keys (left, right, up, down) to move, and press Enter to select.
            f.render_widget(bottom_instruction, main_chunks[2]);
        });
    }

    pub fn handle_key_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                match key_event.code {
                    KeyCode::Up | KeyCode::Down | KeyCode::Left | KeyCode::Right => {
                        self.app.move_selected_button(key_event.code);
                    }
                    KeyCode::Enter => {
                        self.app.click_selected_button();
                    }
                    KeyCode::Char('c') => self.app.reset(),
                    KeyCode::Char('7') => self.app.set_operand(7.0),
                    KeyCode::Char('8') => self.app.set_operand(8.0),
                    KeyCode::Char('9') => self.app.set_operand(9.0),
                    KeyCode::Char('4') => self.app.set_operand(4.0),
                    KeyCode::Char('5') => self.app.set_operand(5.0),
                    KeyCode::Char('6') => self.app.set_operand(6.0),
                    KeyCode::Char('1') => self.app.set_operand(1.0),
                    KeyCode::Char('2') => self.app.set_operand(2.0),
                    KeyCode::Char('3') => self.app.set_operand(3.0),
                    KeyCode::Char('0') => self.app.set_operand(0.0),
                    KeyCode::Char('=') => self.app.calculate(),
                    KeyCode::Char('+') => self.app.set_operator('+'),
                    KeyCode::Char('-') => self.app.set_operator('-'),
                    KeyCode::Char('*') => self.app.set_operator('*'),
                    KeyCode::Char('/') => self.app.set_operator('/'),
                    KeyCode::Char('q') => self.app.quit(),
                    _ => {}
                }
            }
            _ => {}
        };
        Ok(())
    }

    pub fn restore(self) -> io::Result<()> {
        execute!(stdout(), LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Ok(())
    }

    pub fn get_app(&self) -> App {
        return self.app.clone();
    }

    fn active_buttons(&mut self) -> [[bool; 4]; 5] {
        let mut active_states = [[false; 4]; 5];

        if let Some((selected_row, selected_col)) = self.app.get_selected_button() {
            active_states[selected_row][selected_col] = true;
        }

        active_states
    }
}

fn empty<'a>() -> Paragraph<'a> {
    return Paragraph::new("");
}

fn button(text: &str, bg: Color, fg: Color) -> Paragraph {
    Paragraph::new(text)
        .style(Style::default().fg(fg).bg(bg))
        .alignment(Alignment::Center)
}

fn button_primary(text: &str, active: bool) -> Paragraph {
    return button(
        text,
        if active { Color::Red } else { Color::DarkGray },
        Color::White,
    );
}

fn button_secondary(text: &str, active: bool) -> Paragraph {
    return button(
        text,
        if active { Color::Red } else { Color::Gray },
        Color::White,
    );
}

fn input(text: &str) -> Paragraph {
    return button_primary(text, false);
}

fn p_center(text: &str) -> Paragraph {
    return Paragraph::new(text)
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::White));
}

fn layout_buttons_vertical() -> Layout {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
                Constraint::Percentage(20),
            ]
            .as_ref(),
        )
}

fn layout_button_horizontal() -> Layout {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
                Constraint::Percentage(25),
            ]
            .as_ref(),
        )
}
