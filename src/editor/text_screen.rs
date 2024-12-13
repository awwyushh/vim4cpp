use crate::editor::modes::EditorMode;
use crate::editor::snippets::default_snippets;
use crate::editor::io;
use crate::utils::vec2::Vec2;

use cursive::event::{Event, EventResult, Key};
use cursive::theme::{BaseColor, Color, ColorStyle};
use cursive::Vec2 as CursiveVec2;
use cursive::{Printer, View};
use std::collections::HashMap;
use std::time::{Instant, SystemTime};
use chrono::{DateTime, Local};

pub struct TextScreen {
    pub mode: EditorMode,
    pub content: Vec<String>,
    pub cursor: Vec2,
    pub clipboard: String,
    pub search_query: String,
    pub search_mode: bool,
    pub filename: Option<String>,
    pub suggestions: Vec<String>,
    pub showing_suggestions: bool,
    pub selected_suggestion: usize,
    pub last_tab_time: Option<Instant>,
    pub snippets: HashMap<String, String>,
}

impl Default for TextScreen {
    fn default() -> Self {
        TextScreen {
            mode: EditorMode::Normal,
            content: vec![String::new()],
            cursor: Vec2::new(0, 0),
            clipboard: String::new(),
            search_query: String::new(),
            search_mode: false,
            filename: None,
            suggestions: Vec::new(),
            showing_suggestions: false,
            selected_suggestion: 0,
            last_tab_time: None,
            snippets: default_snippets(),
        }
    }
}

impl TextScreen {
    const KEYWORDS: &'static [&'static str] = &[
        "auto", "break", "case", "char", "const", "continue", "default", "do",
        "double", "else", "enum", "extern", "float", "for", "goto", "if",
        "int", "long", "register", "return", "short", "signed", "sizeof", "static",
        "struct", "switch", "typedef", "union", "unsigned", "void", "volatile", "while",
        "class", "namespace", "template", "public", "private", "protected", "using",
        "include", "define", "ifdef", "ifndef", "endif",
    ];

    pub fn load_file(&mut self, filename: &str) {
        if let Ok(lines) = io::load_file(filename) {
            self.content = lines;
            self.filename = Some(filename.to_string());
        }
    }

    pub fn save_file(&self) -> std::io::Result<()> {
        if let Some(filename) = &self.filename {
            io::save_file(filename, &self.content)?;
        }
        Ok(())
    }

    fn current_line(&self) -> &String {
        &self.content[self.cursor.y]
    }

    fn get_current_word(&self) -> String {
        let line = self.current_line();
        let mut start = self.cursor.x;
        
        while start > 0 && line.chars().nth(start - 1).map_or(false, |c| c.is_alphanumeric() || c == '_') {
            start -= 1;
        }
        
        line[start..self.cursor.x].to_string()
    }

    // ... rest of the TextScreen implementation ...
    // (including update_suggestions, insert_suggestion, and other methods)
}

impl View for TextScreen {
    fn draw(&self, printer: &Printer) {
        // Title
        let title = "VIM 4 C++";
        let title_pos = (printer.size.x - title.len()) / 2;
        printer.with_color(ColorStyle::new(
            Color::Light(BaseColor::Green),
            Color::Dark(BaseColor::Black)
        ), |printer| {
            printer.print((title_pos, 0), title);
        });

        // Mode and time
        let mode_str = match self.mode {
            EditorMode::Normal => "NORMAL",
            EditorMode::Insert => "INSERT",
            EditorMode::Command => "COMMAND",
        };
        printer.print((0, 1), mode_str);

        let now: DateTime<Local> = SystemTime::now().into();
        let time_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let time_pos = printer.size.x - time_str.len();
        printer.print((time_pos, 1), &time_str);

        // Calculate visible area
        let content_height = printer.size.y - 3;
        let scroll_offset = if self.cursor.y >= content_height {
            self.cursor.y - content_height + 1
        } else {
            0
        };

        // Draw content
        for (i, line) in self.content.iter()
            .skip(scroll_offset)
            .take(content_height)
            .enumerate()
        {
            // Print the entire line including leading spaces
            printer.print((0, i + 2), line);

            // Color the words after printing the full line
            let mut x = 0;
            for word in line.split_whitespace() {
                let word_start = line[x..].find(word).unwrap_or(0) + x;
                let color = if Self::KEYWORDS.contains(&word) {
                    ColorStyle::new(Color::Light(BaseColor::Magenta), Color::Dark(BaseColor::Black))
                } else {
                    ColorStyle::new(Color::Light(BaseColor::White), Color::Dark(BaseColor::Black))
                };
                printer.with_color(color, |printer| {
                    printer.print((word_start, i + 2), word);
                });
                x = word_start + word.len();
            }
        }

        // Draw suggestions if active
        if self.showing_suggestions && !self.suggestions.is_empty() {
            let suggestion_y = (self.cursor.y - scroll_offset + 3).min(printer.size.y - 1);
            let mut suggestion_x = self.cursor.x;
            
            for (i, suggestion) in self.suggestions.iter().enumerate() {
                let style = if i == self.selected_suggestion {
                    ColorStyle::new(Color::Light(BaseColor::Black), Color::Light(BaseColor::White))
                } else {
                    ColorStyle::new(Color::Light(BaseColor::White), Color::Dark(BaseColor::Black))
                };
                
                printer.with_color(style, |printer| {
                    printer.print((suggestion_x, suggestion_y), suggestion);
                });
                
                suggestion_x += suggestion.len() + 1;
            }
        }

        // Draw cursor
        if self.cursor.y >= scroll_offset && self.cursor.y < scroll_offset + content_height {
            printer.print(
                (self.cursor.x, self.cursor.y - scroll_offset + 2),
                "█"
            );
        }
    }

    fn on_event(&mut self, event: Event) -> EventResult {
        match self.mode {
            EditorMode::Insert => match event {
                Event::Char(c) => {
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    
                    match c {
                        '(' => {
                            self.content[y].insert_str(x, "()");
                            self.cursor.x = x + 1;
                        }
                        '{' => {
                            self.content[y].insert_str(x, "{}");
                            self.cursor.x = x + 1;
                        }
                        '[' => {
                            self.content[y].insert_str(x, "[]");
                            self.cursor.x = x + 1;
                        }
                        _ => {
                            self.content[y].insert(x, c);
                            self.cursor.x = x + 1;
                        }
                    }
                    self.update_suggestions();
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Tab) => {
                    let now = Instant::now();
                    
                    if let Some(last_time) = self.last_tab_time {
                        if now.duration_since(last_time).as_millis() < 300 {
                            if self.showing_suggestions && !self.suggestions.is_empty() {
                                let x = self.cursor.x;
                                let y = self.cursor.y;
                                let current_word = self.get_current_word();
                                let suggestion = &self.suggestions[self.selected_suggestion];
                                
                                // Remove the partial word
                                for _ in 0..current_word.len() {
                                    self.content[y].remove(x - current_word.len());
                                }
                                self.cursor.x -= current_word.len();
                                
                                // Insert the full suggestion
                                self.content[y].insert_str(self.cursor.x, suggestion);
                                self.cursor.x += suggestion.len();
                                
                                self.showing_suggestions = false;
                                self.suggestions.clear();
                                self.last_tab_time = None;
                                return EventResult::Consumed(None);
                            }
                        }
                    }
                    
                    self.last_tab_time = Some(now);
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    let spaces = "    ";
                    self.content[y].insert_str(x, spaces);
                    self.cursor.x = x + 4;
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Enter) => {
                    if self.showing_suggestions {
                        self.insert_suggestion();
                    } else {
                        let x = self.cursor.x;
                        let y = self.cursor.y;
                        let new_line = self.content[y][x..].to_string();
                        self.content[y].truncate(x);
                        self.content.insert(y + 1, new_line);
                        self.cursor.y += 1;
                        self.cursor.x = 0;
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Up) => {
                    if self.showing_suggestions {
                        self.navigate_suggestions(-1);
                        EventResult::Consumed(None)
                    } else if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                        let line_len = self.current_line().len();
                        self.cursor.x = self.cursor.x.min(line_len);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
                Event::Key(Key::Down) => {
                    if self.showing_suggestions {
                        self.navigate_suggestions(1);
                        EventResult::Consumed(None)
                    } else if self.cursor.y < self.content.len() - 1 {
                        self.cursor.y += 1;
                        let line_len = self.current_line().len();
                        self.cursor.x = self.cursor.x.min(line_len);
                        EventResult::Consumed(None)
                    } else {
                        EventResult::Ignored
                    }
                }
                Event::Key(Key::Left) => {
                    if self.cursor.x > 0 {
                        self.cursor.x -= 1;
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Right) => {
                    if self.cursor.x < self.current_line().len() {
                        self.cursor.x += 1;
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Backspace) => {
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    if x > 0 {
                        self.content[y].remove(x - 1);
                        self.cursor.x -= 1;
                        self.update_suggestions();
                    } else if y > 0 {
                        let line = self.content.remove(y);
                        self.cursor.y -= 1;
                        self.cursor.x = self.content[y - 1].len();
                        self.content[y - 1].push_str(&line);
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Esc) => {
                    self.mode = EditorMode::Normal;
                    if self.cursor.x > 0 {
                        self.cursor.x -= 1;
                    }
                    EventResult::Consumed(None)
                }
                _ => EventResult::Ignored,
            },
            EditorMode::Normal => match event {
                Event::Char('i') => {
                    self.mode = EditorMode::Insert;
                    EventResult::Consumed(None)
                }
                Event::Char(':') => {
                    self.mode = EditorMode::Command;
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Left) | Event::Char('h') => {
                    if self.cursor.x > 0 {
                        self.cursor.x -= 1;
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Right) | Event::Char('l') => {
                    if self.cursor.x < self.current_line().len() {
                        self.cursor.x += 1;
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Up) | Event::Char('k') => {
                    if self.cursor.y > 0 {
                        self.cursor.y -= 1;
                        let line_len = self.current_line().len();
                        self.cursor.x = self.cursor.x.min(line_len);
                    }
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Down) | Event::Char('j') => {
                    if self.cursor.y < self.content.len() - 1 {
                        self.cursor.y += 1;
                        let line_len = self.current_line().len();
                        self.cursor.x = self.cursor.x.min(line_len);
                    }
                    EventResult::Consumed(None)
                }
                _ => EventResult::Ignored,
            },
            EditorMode::Command => match event {
                Event::Key(Key::Enter) => {
                    match self.search_query.as_str() {
                        "w" => {
                            self.save_file().ok();
                            self.mode = EditorMode::Normal;
                        },
                        "q" => {
                            return EventResult::with_cb(|s| s.quit());
                        },
                        "wq" | "x" => {
                            self.save_file().ok();
                            return EventResult::with_cb(|s| s.quit());
                        },
                        _ => {
                            self.mode = EditorMode::Normal;
                        }
                    }
                    self.search_query.clear();
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Esc) => {
                    self.mode = EditorMode::Normal;
                    self.search_query.clear();
                    EventResult::Consumed(None)
                }
                Event::Char(c) => {
                    self.search_query.push(c);
                    EventResult::Consumed(None)
                }
                Event::Key(Key::Backspace) => {
                    self.search_query.pop();
                    EventResult::Consumed(None)
                }
                _ => EventResult::Ignored,
            },
        }
    }

    fn required_size(&mut self, _: CursiveVec2) -> CursiveVec2 {
        CursiveVec2::new(80, 24)
    }
}

impl TextScreen {
    fn navigate_suggestions(&mut self, direction: i32) {
        if !self.suggestions.is_empty() {
            let len = self.suggestions.len();
            self.selected_suggestion = if direction > 0 {
                (self.selected_suggestion + 1) % len
            } else {
                (self.selected_suggestion + len - 1) % len
            };
        }
    }

    fn update_suggestions(&mut self) {
        let current_word = self.get_current_word();
        if current_word.len() >= 2 {
            // First check snippets
            let snippet_matches: Vec<String> = self.snippets.keys()
                .filter(|&k| k.starts_with(&current_word))
                .cloned()
                .collect();

            if !snippet_matches.is_empty() {
                self.suggestions = snippet_matches;
                self.showing_suggestions = true;
                self.selected_suggestion = 0;
                return;
            }

            // If no snippet matches, check keywords
            self.suggestions = Self::KEYWORDS
                .iter()
                .filter(|&kw| kw.starts_with(&current_word))
                .map(|&s| s.to_string())
                .collect();
            self.showing_suggestions = !self.suggestions.is_empty();
            self.selected_suggestion = 0;
        } else {
            self.showing_suggestions = false;
            self.suggestions.clear();
        }
    }

    fn insert_suggestion(&mut self) {
        if self.showing_suggestions && !self.suggestions.is_empty() {
            let suggestion = self.suggestions[self.selected_suggestion].clone();
            
            // Check if this is a snippet
            if let Some(snippet_content) = self.snippets.get(&suggestion) {
                let current_word = self.get_current_word();
                
                // Remove the current word
                for _ in 0..current_word.len() {
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    if x > 0 {
                        self.content[y].remove(x - 1);
                        self.cursor.x -= 1;
                    }
                }
                
                // Insert the snippet content
                let lines: Vec<&str> = snippet_content.lines().collect();
                if !lines.is_empty() {
                    // Insert first line at current position
                    let x = self.cursor.x;
                    let y = self.cursor.y;
                    self.content[y].insert_str(x, lines[0]);
                    self.cursor.x += lines[0].len();
                    
                    // Insert remaining lines
                    for line in lines.iter().skip(1) {
                        self.cursor.y += 1;
                        self.content.insert(self.cursor.y, line.to_string());
                    }
                    
                    // Position cursor inside main function
                    for (i, line) in self.content.iter().enumerate() {
                        if line.contains("main() {") {
                            self.cursor.y = i + 1;
                            self.cursor.x = 4; // 4 spaces indentation
                            break;
                        }
                    }
                }
            } else {
                // Regular suggestion (keyword)
                let current_word = self.get_current_word();
                let remaining = suggestion[current_word.len()..].to_string();
                
                let x = self.cursor.x;
                let y = self.cursor.y;
                self.content[y].insert_str(x, &remaining);
                self.cursor.x += remaining.len();
            }
            self.showing_suggestions = false;
            self.suggestions.clear();
        }
    }
}
