use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{egui, App, NativeOptions};

struct MyApp {
  clipboard_text: String,
  inverted_text: String,
  clipboard: ClipboardContext,
  error_message: Option<String>,
}

impl Default for MyApp {
  fn default() -> Self {
    Self {
      clipboard_text: String::new(),
      inverted_text: String::new(),
      clipboard: ClipboardContext::new().unwrap(),
      error_message: None,
    }
  }
}

impl App for MyApp {
  fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    match self.clipboard.get_contents() {
      Ok(contents) => {
        if contents != self.clipboard_text {
          self.clipboard_text = contents.clone();
          self.inverted_text = invert_layout(&contents);
        }
        self.error_message = None;
      }
      Err(e) => {
        self.error_message = Some(format!("Ошибка доступа к буферу обмена: {}", e));
      }
    }

    egui::CentralPanel::default().show(ctx, |ui| {
      ui.horizontal(|ui| {
        if ui.button("Скопировать инвертированный текст").clicked() {
          if let Err(e) = self.clipboard.set_contents(self.inverted_text.clone()) {
            self.error_message = Some(format!("Ошибка записи в буфер обмена: {}", e));
          } else {
            self.error_message = None;
          }
        }
        if let Some(err) = &self.error_message {
          ui.colored_label(egui::Color32::RED, err);
        }
      });

      ui.separator();

      ui.heading("Буфер обмена");
      ui.add(
        egui::TextEdit::multiline(&mut self.clipboard_text)
          .desired_rows(5)
          .desired_width(f32::INFINITY)
          .frame(true),
      );

      ui.separator();

      ui.heading("Инвертированная раскладка");
      ui.add(
        egui::TextEdit::multiline(&mut self.inverted_text)
          .desired_rows(5)
          .desired_width(f32::INFINITY)
          .frame(true),
      );
    });

    ctx.request_repaint_after(std::time::Duration::from_millis(500));
  }
}

fn invert_layout(input: &str) -> String {
  let mapping = [
    ('q', 'й'),
    ('w', 'ц'),
    ('e', 'у'),
    ('r', 'к'),
    ('t', 'е'),
    ('y', 'н'),
    ('u', 'г'),
    ('i', 'ш'),
    ('o', 'щ'),
    ('p', 'з'),
    ('[', 'х'),
    (']', 'ъ'),
    ('a', 'ф'),
    ('s', 'ы'),
    ('d', 'в'),
    ('f', 'а'),
    ('g', 'п'),
    ('h', 'р'),
    ('j', 'о'),
    ('k', 'л'),
    ('l', 'д'),
    (';', 'ж'),
    ('\'', 'э'),
    ('z', 'я'),
    ('x', 'ч'),
    ('c', 'с'),
    ('v', 'м'),
    ('b', 'и'),
    ('n', 'т'),
    ('m', 'ь'),
    (',', 'б'),
    ('.', 'ю'),
  ];

  input
    .chars()
    .map(|c| {
      if c.is_ascii_lowercase() {
        mapping
          .iter()
          .find(|(en, _)| *en == c)
          .map(|&(_, ru)| ru)
          .unwrap_or(c)
      } else if c.is_ascii_uppercase() {
        let lower = c.to_ascii_lowercase();
        mapping
          .iter()
          .find(|(en, _)| *en == lower)
          .map(|&(_, ru)| ru.to_ascii_uppercase())
          .unwrap_or(c)
      } else {
        c
      }
    })
    .collect()
}

fn main() {
  let app = MyApp::default();
  let native_options = NativeOptions {
    initial_window_size: Some(egui::vec2(600.0, 400.0)),
    ..Default::default()
  };
  eframe::run_native(
    "Clipboard Viewer",
    native_options,
    Box::new(|_cc| Box::new(app)),
  );
}
