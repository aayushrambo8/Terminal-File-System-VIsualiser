use crossterm::event::{KeyEvent, MouseButton, MouseEventKind};

use crate::app::App;

pub fn handle_key_event(app: &mut App, key: KeyEvent) -> Result<bool, std::io::Error> {
    match key.code {
        crossterm::event::KeyCode::Char('q') => return Ok(true),
        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down if app.is_context_menu_open() => {
            app.move_context_menu(1);
        }
        crossterm::event::KeyCode::Char('j') | crossterm::event::KeyCode::Down => {
            app.move_selection(1);
        }
        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up if app.is_context_menu_open() => {
            app.move_context_menu(-1);
        }
        crossterm::event::KeyCode::Char('k') | crossterm::event::KeyCode::Up => {
            app.move_selection(-1);
        }
        crossterm::event::KeyCode::PageDown => app.page_down(10),
        crossterm::event::KeyCode::PageUp => app.page_up(10),
        crossterm::event::KeyCode::Home => app.move_to_start(),
        crossterm::event::KeyCode::End => app.move_to_end(),
        crossterm::event::KeyCode::Char('l') | crossterm::event::KeyCode::Right => {
            app.enter_selected();
        }
        crossterm::event::KeyCode::Char('h') | crossterm::event::KeyCode::Left => {
            app.go_parent();
        }
        crossterm::event::KeyCode::Char('r') => app.refresh(),
        crossterm::event::KeyCode::Char('s') => app.cycle_sort(),
        crossterm::event::KeyCode::Char('n') => app.create_directory("new-folder"),
        crossterm::event::KeyCode::Char('m') => app.rename_selected("renamed-item"),
        crossterm::event::KeyCode::Char('d') => app.delete_selected(),
        crossterm::event::KeyCode::Char('c') => app.toggle_context_menu(),
        crossterm::event::KeyCode::Enter if app.is_context_menu_open() => app.apply_context_menu(),
        crossterm::event::KeyCode::Enter => app.apply_context_menu(),
        crossterm::event::KeyCode::Esc => {
            if app.is_context_menu_open() {
                app.close_context_menu();
            } else {
                return Ok(true);
            }
        }
        _ => {}
    }
    Ok(false)
}

pub fn handle_mouse_event(app: &mut App, mouse: crossterm::event::MouseEvent) {
    if matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
        let row = mouse.row.saturating_sub(4) as usize;
        app.select_index(row);
    }
}
