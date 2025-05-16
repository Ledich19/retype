use std::{io, time::Duration};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use tui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use copypasta::{ClipboardContext, ClipboardProvider};

pub fn run_ui() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let res = ui_loop(&mut terminal).map_err(|e| Box::new(e) as Box<dyn std::error::Error>);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    res
}

fn ui_loop(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut input = String::new();
    let mut selected_text = String::new();

    // Инициализация буфера обмена
    let mut clipboard = ClipboardContext::new().unwrap_or_else(|_| {
        panic!("Не удалось инициализировать буфер обмена. Проверьте окружение.");
    });

    // Получаем текст из буфера один раз
    let mut clipboard_text = clipboard.get_contents().unwrap_or_default();

    loop {
        terminal.draw(|f| {
            let size = f.size();

            // Вертикальный лэйаут: три блока по высоте
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints(
                    [
                        Constraint::Length(5), // буфер обмена
                        Constraint::Length(5), // выделенный текст
                        Constraint::Min(0),    // ввод
                    ]
                    .as_ref(),
                )
                .split(size);

            let clipboard_block = Block::default().title("Буфер обмена").borders(Borders::ALL);
            let clipboard_paragraph = Paragraph::new(clipboard_text.as_ref()).block(clipboard_block);
            f.render_widget(clipboard_paragraph, chunks[0]);

            let selected_block = Block::default().title("Выделенный текст").borders(Borders::ALL);
            let selected_paragraph = Paragraph::new(selected_text.as_ref()).block(selected_block);
            f.render_widget(selected_paragraph, chunks[1]);

            let input_block = Block::default().title("Ввод с клавиатуры").borders(Borders::ALL);
            let input_paragraph = Paragraph::new(input.as_ref()).block(input_block);
            f.render_widget(input_paragraph, chunks[2]);
        })?;

        if event::poll(Duration::from_millis(100))? {
            match event::read()? {
                Event::Key(key) => match key.code {
                    KeyCode::Char(c) => input.push(c),
                    KeyCode::Backspace => {
                        input.pop();
                    }
                    KeyCode::Esc | KeyCode::Char('q') => break,
                    KeyCode::F(1) => {
                        // Копировать ввод в буфер обмена
                        if clipboard.set_contents(input.clone()).is_ok() {
                            clipboard_text = input.clone();
                        }
                    }
                    KeyCode::F(2) => {
                        // Копировать весь ввод в выделенный текст
                        selected_text = input.clone();
                    }
                    KeyCode::F(3) => {
                        // Обновить буфер обмена из системы
                        clipboard_text = clipboard.get_contents().unwrap_or_default();
                    }
                    _ => {}
                },
                _ => {}
            }
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run_ui() {
        eprintln!("Ошибка: {}", e);
    }
}
