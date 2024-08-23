#[cfg(test)]
use ratatui::crossterm::event::KeyCode;

#[test]
fn test_set_operand() {
    use crate::app::App;
    let mut app = App::new();
    app.set_operand(5.0);
    assert_eq!(app.get_operand1(), Some("5".to_string()));

    app.set_operator('+');
    app.set_operand(3.0);
    assert_eq!(app.get_operand2(), Some("3".to_string()));
}

#[test]
fn test_set_operator() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operator('+');
    assert_eq!(app.get_operator(), Some('+'));

    app.set_operator('-');
    assert_eq!(app.get_operator(), Some('+'));
}

#[test]
fn test_calculate_addition() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operand(5.0);
    app.set_operator('+');
    app.set_operand(3.0);
    app.calculate();
    assert_eq!(app.get_result(), Some(8.0));
}

#[test]
fn test_calculate_subtraction() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operand(5.0);
    app.set_operator('-');
    app.set_operand(3.0);
    app.calculate();
    assert_eq!(app.get_result(), Some(2.0));
}

#[test]
fn test_calculate_multiplication() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operand(5.0);
    app.set_operator('*');
    app.set_operand(3.0);
    app.calculate();
    assert_eq!(app.get_result(), Some(15.0));
}

#[test]
fn test_calculate_division() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operand(9.0);
    app.set_operator('/');
    app.set_operand(3.0);
    app.calculate();
    assert_eq!(app.get_result(), Some(3.0));
}

#[test]
fn test_reset() {
    use crate::app::App;

    let mut app = App::new();
    app.set_operand(9.0);
    app.set_operator('*');
    app.set_operand(3.0);
    app.calculate();

    app.reset();
    assert_eq!(app.get_operand1(), None);
    assert_eq!(app.get_operator(), None);
    assert_eq!(app.get_operand2(), None);
    assert_eq!(app.get_result(), None);
}

#[test]
fn test_button_navigation() {
    use crate::app::App;

    let mut app = App::new();
    app.move_selected_button(KeyCode::Down);
    assert_eq!(app.get_selected_button(), Some((1, 0)));

    app.move_selected_button(KeyCode::Right);
    assert_eq!(app.get_selected_button(), Some((1, 1)));

    app.move_selected_button(KeyCode::Down);
    assert_eq!(app.get_selected_button(), Some((2, 1)));
}

#[test]
fn test_button_click() {
    use crate::app::App;
    
    let mut app = App::new();
    app.move_selected_button(KeyCode::Down); // Move to "7"
    app.click_selected_button();
    assert_eq!(app.get_operand1(), Some("7".to_string()));

    app.move_selected_button(KeyCode::Right); // Move to "8"
    app.click_selected_button();
    assert_eq!(app.get_operand1(), Some("78".to_string()));

    app.move_selected_button(KeyCode::Right); // Move to "9"
    app.click_selected_button();
    assert_eq!(app.get_operand1(), Some("789".to_string()));
}
