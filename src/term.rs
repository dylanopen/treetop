use std::{io::{Stdout, Write}, sync::Mutex};

use crossterm::terminal;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref STDOUT: Mutex<Stdout> = Mutex::new(std::io::stdout());
}

pub fn enable_raw_mode() {
    terminal::enable_raw_mode().expect("Failed to enable raw mode");
}

pub fn disable_raw_mode() {
    terminal::disable_raw_mode().expect("Failed to disable raw mode");
}

pub fn enter_alternate_screen() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)
        .expect("Failed to enter alternate screen");
}

pub fn exit_alternate_screen() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::LeaveAlternateScreen)
        .expect("Failed to leave alternate screen");
}
pub fn clear_screen() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::All))
        .expect("Failed to clear screen");
}

pub fn flush_stdout() {
    let mut stdout = STDOUT.lock().unwrap();
    stdout.flush().expect("Failed to flush stdout");
}

pub fn get_terminal_size() -> (u16, u16) {
    terminal::size().expect("Failed to get terminal size")
}

pub fn get_cursor_position() -> (u16, u16) {
    crossterm::cursor::position().expect("Failed to get cursor position")
}

pub fn set_cursor_position(x: u16, y: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::MoveTo(x, y))
        .expect("Failed to set cursor position");
}

pub fn hide_cursor() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::Hide).expect("Failed to hide cursor");
}

pub fn show_cursor() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::Show).expect("Failed to show cursor");
}

pub fn clear_current_line() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::Clear(crossterm::terminal::ClearType::CurrentLine))
        .expect("Failed to clear line");
}

pub fn move_cursor_up(rows: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::MoveUp(rows))
        .expect("Failed to move cursor up");
}

pub fn move_cursor_down(rows: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::MoveDown(rows))
        .expect("Failed to move cursor down");
}

pub fn move_cursor_left(cols: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::MoveLeft(cols))
        .expect("Failed to move cursor left");
}

pub fn move_cursor_right(cols: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::MoveRight(cols))
        .expect("Failed to move cursor right");
}

pub fn save_cursor_position() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::SavePosition)
        .expect("Failed to save cursor position");
}

pub fn restore_cursor_position() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::cursor::RestorePosition)
        .expect("Failed to restore cursor position");
}

pub fn set_title(title: &str) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::SetTitle(title))
        .expect("Failed to set terminal title");
}

pub fn clear_title() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::SetTitle(""))
        .expect("Failed to clear terminal title");
}

pub fn scroll_up(lines: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::ScrollUp(lines))
        .expect("Failed to scroll up");
}

pub fn scroll_down(lines: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::ScrollDown(lines))
        .expect("Failed to scroll down");
}

pub fn set_size(cols: u16, rows: u16) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::terminal::SetSize(cols, rows))
        .expect("Failed to set terminal size");
}

pub fn set_background_color(color: crossterm::style::Color) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::style::SetBackgroundColor(color))
        .expect("Failed to set background color");
}

pub fn set_foreground_color(color: crossterm::style::Color) {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::style::SetForegroundColor(color))
        .expect("Failed to set foreground color");
}

pub fn reset_colors() {
    let mut stdout = STDOUT.lock().unwrap();
    crossterm::execute!(stdout, crossterm::style::ResetColor)
        .expect("Failed to reset colors");
}

