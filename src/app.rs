use ratatui::crossterm::event::KeyCode;

#[derive(Debug, Clone)]
pub struct App {
    operand1: Option<String>,
    operator: Option<char>,
    operand2: Option<String>,
    result: Option<f64>,
    exit: bool,
    selected_button: Option<(usize, usize)>,
}

impl App {
    pub fn new() -> App {
        App {
            operand1: None,
            operator: None,
            operand2: None,
            result: None,
            exit: false,
            selected_button: None,
        }
    }

    pub fn reset(&mut self) {
        self.operand1 = None;
        self.operator = None;
        self.operand2 = None;
        self.result = None;
    }

    pub fn set_operand(&mut self, digit: f64) {
        match self.operator {
            Some(_) => {
                self.operand2 = Some(format!(
                    "{}{}",
                    self.operand2.as_ref().unwrap_or(&String::from("")),
                    digit
                ))
            }
            None => {
                self.operand1 = Some(format!(
                    "{}{}",
                    self.operand1.as_ref().unwrap_or(&String::from("")),
                    digit
                ))
            }
        }
    }

    pub fn set_operator(&mut self, op: char) {
        match self.operator {
            Some(_) => {}
            None => self.operator = Some(op),
        };
    }

    pub fn calculate(&mut self) {
        if let Some(op) = self.operator {
            if let Some(op1) = self.operand1.clone() {
                if let Some(op2) = self.operand2.clone() {
                    let operand1 = op1.parse::<f64>().unwrap();
                    let operand2: f64 = op2.parse::<f64>().unwrap();

                    match op {
                        '+' => self.result = Some(operand1 + operand2),
                        '-' => self.result = Some(operand1 - operand2),
                        '*' => self.result = Some(operand1 * operand2),
                        '/' => self.result = Some(operand1 / operand2),
                        _ => {}
                    }
                }
            }
        }
    }

    pub fn get_display(&self) -> String {
        if let Some(res) = self.result {
            return format!("{}", res);
        }
        if let Some(op2) = self.operand2.clone() {
            return format!(
                "{} {} {}",
                self.operand1.clone().unwrap(),
                self.operator.unwrap(),
                op2
            );
        }
        if let Some(op1) = self.operand1.clone() {
            if let Some(operator) = self.operator {
                return format!("{} {}", op1, operator);
            }
            return format!("{}", op1);
        }
        return "0".to_string();
    }

    pub fn get_exit(&self) -> bool {
        return self.exit;
    }

    pub fn quit(&mut self) {
        self.exit = true;
    }

    pub fn get_selected_button(&self) -> Option<(usize, usize)> {
        return self.selected_button;
    }

    pub fn move_selected_button(&mut self, direction: KeyCode) {
        if let Some((row, col)) = self.selected_button {
            let (new_row, new_col) = match direction {
                KeyCode::Up if row > 0 => {
                    if col < 3 && row == 1 {
                        (row, col)
                    } else {
                        (row - 1, col)
                    }
                }
                KeyCode::Down if row < 4 => {
                    if col == 1 && row == 3 {
                        (row, col)
                    } else {
                        (row + 1, col)
                    }
                }
                KeyCode::Left if col > 0 => {
                    if col == 2 && row == 4 {
                        (row, col)
                    } else {
                        (row, col - 1)
                    }
                }
                KeyCode::Right if col < 3 => {
                    if col == 0 && row == 4 {
                        (row, col)
                    } else {
                        (row, col + 1)
                    }
                }
                _ => (row, col),
            };
            return self.selected_button = Some((new_row, new_col));
        }
        return self.selected_button = Some((1, 0));
    }

    pub fn click_selected_button(&mut self) {
        if let Some((row, col)) = self.selected_button {
            match (row, col) {
                (0, 3) => self.reset(),
                (1, 0) => self.set_operand(7.0),
                (1, 1) => self.set_operand(8.0),
                (1, 2) => self.set_operand(9.0),
                (1, 3) => self.set_operator('/'),
                (2, 0) => self.set_operand(4.0),
                (2, 1) => self.set_operand(5.0),
                (2, 2) => self.set_operand(6.0),
                (2, 3) => self.set_operator('*'),
                (3, 0) => self.set_operand(1.0),
                (3, 1) => self.set_operand(2.0),
                (3, 2) => self.set_operand(3.0),
                (3, 3) => self.set_operator('-'),
                (4, 0) => self.set_operand(0.0),
                (4, 2) => self.calculate(),
                (4, 3) => self.set_operator('+'),
                _ => {}
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_operand1(&self) -> Option<String> {
        return self.operand1.clone();
    }

    #[allow(dead_code)]
    pub fn get_operand2(&self) -> Option<String> {
        return self.operand2.clone();
    }

    #[allow(dead_code)]
    pub fn get_operator(&self) -> Option<char> {
        return self.operator.clone();
    }

    #[allow(dead_code)]
    pub fn get_result(&self) -> Option<f64> {
        return self.result.clone();
    }
}
