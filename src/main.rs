#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // releaseビルドでのみ適用
// #![windows_subsystem = "windows"] // 常に適用する場合はこちらを使用

use eframe::egui;
use serde::{Deserialize, Serialize};
use std::fs;
use chrono::{DateTime, Local, Datelike};

#[derive(Serialize, Deserialize, Default)]
struct TodoItem {
    text: String,
    completed: bool,
    due_date: Option<i64>,  // Unix timestamp
}

#[derive(Serialize, Deserialize, Default)]
struct Timer {
    start_time: Option<i64>,    // タイマー開始時刻
    duration: Option<i64>,      // 設定時間（秒）
    is_running: bool,
}

#[derive(Serialize, Deserialize, Default)]
struct TodoApp {
    items: Vec<TodoItem>,
    new_item_text: String,
    new_item_date: Option<i64>,
    show_date_picker: bool,
    selected_year: i32,
    selected_month: u32,
    selected_day: u32,
    timer: Timer,
}

impl Timer {
    fn start(&mut self, minutes: i64) {
        self.start_time = Some(Local::now().timestamp());
        self.duration = Some(minutes * 60);
        self.is_running = true;
    }

    fn stop(&mut self) {
        self.start_time = None;
        self.duration = None;
        self.is_running = false;
    }

    fn remaining_time(&self) -> Option<i64> {
        if !self.is_running {
            return None;
        }
        let now = Local::now().timestamp();
        let elapsed = now - self.start_time?;
        let duration = self.duration?;
        Some(duration - elapsed)
    }

    fn format_remaining_time(&self) -> String {
        if let Some(remaining) = self.remaining_time() {
            if remaining <= 0 {
                return "時間終了！".to_string();
            }
            let minutes = remaining / 60;
            let seconds = remaining % 60;
            format!("{}:{:02}", minutes, seconds)
        } else {
            "--:--".to_string()
        }
    }
}

impl TodoApp {
    fn new() -> Self {
        if let Ok(file) = fs::read_to_string("todos.json") {
            if let Ok(items) = serde_json::from_str(&file) {
                return TodoApp {
                    items,
                    new_item_text: String::new(),
                    new_item_date: None,
                    show_date_picker: false,
                    selected_year: Local::now().year(),
                    selected_month: Local::now().month(),
                    selected_day: Local::now().day(),
                    timer: Timer::default(),
                };
            }
        }
        let now = Local::now();
        TodoApp {
            items: Vec::new(),
            new_item_text: String::new(),
            new_item_date: None,
            show_date_picker: false,
            selected_year: now.year(),
            selected_month: now.month(),
            selected_day: now.day(),
            timer: Timer::default(),
        }
    }

    fn save_to_file(&self) {
        if let Ok(json) = serde_json::to_string_pretty(&self.items) {
            let _ = fs::write("todos.json", json);
        }
    }

    fn format_date(timestamp: i64) -> String {
        let dt = DateTime::from_timestamp(timestamp, 0).unwrap();
        dt.format("%Y-%m-%d").to_string()
    }

    fn get_task_color(&self, due_date: Option<i64>) -> egui::Color32 {
        if let Some(due) = due_date {
            let now = Local::now().timestamp();
            if due < now {
                egui::Color32::from_rgb(255, 100, 100) // 期限超過は赤
            } else if (due - now) < 24 * 60 * 60 {
                egui::Color32::from_rgb(255, 200, 0) // 24時間以内は黄色
            } else {
                egui::Color32::GRAY
            }
        } else {
            egui::Color32::GRAY
        }
    }

    fn sort_tasks(&mut self) {
        self.items.sort_by(|a, b| {
            // 1. 未完了タスクを先に
            if a.completed != b.completed {
                return a.completed.cmp(&b.completed);
            }
            // 2. 期限がないタスクは後ろに
            match (a.due_date, b.due_date) {
                (None, Some(_)) => std::cmp::Ordering::Greater,
                (Some(_), None) => std::cmp::Ordering::Less,
                (Some(a_due), Some(b_due)) => a_due.cmp(&b_due),
                (None, None) => std::cmp::Ordering::Equal,
            }
        });
    }
}

impl eframe::App for TodoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Local::now();

        // ESCキーの検知
        if self.show_date_picker && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            self.show_date_picker = false;
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Todoリスト");

            // 現在時刻とタイマーの表示
            ui.horizontal(|ui| {
                ui.label(format!("現在時刻: {}", now.format("%Y-%m-%d %H:%M:%S")));
                ui.add_space(20.0);

                // タイマー表示と操作ボタン
                if self.timer.is_running {
                    ui.label(format!("残り時間: {}", self.timer.format_remaining_time()));
                    if ui.button("⏹ 停止").clicked() {
                        self.timer.stop();
                    }

                    // タイマー終了チェック
                    if let Some(remaining) = self.timer.remaining_time() {
                        if remaining <= 0 {
                            self.timer.stop();
                        }
                    }
                } else {
                    ui.label("タイマー: ");
                    if ui.button("15分").clicked() {
                        self.timer.start(15);
                    }
                    if ui.button("30分").clicked() {
                        self.timer.start(30);
                    }
                    if ui.button("1時間").clicked() {
                        self.timer.start(60);
                    }
                }
            });
            ui.add_space(8.0);

            // 一括操作ボタン
            ui.horizontal(|ui| {
                if ui.add_sized([120.0, 24.0], egui::Button::new("すべてチェック")).clicked() {
                    for item in &mut self.items {
                        item.completed = true;
                    }
                    self.sort_tasks();
                    self.save_to_file();
                }
                if ui.add_sized([120.0, 24.0], egui::Button::new("すべて未チェック")).clicked() {
                    for item in &mut self.items {
                        item.completed = false;
                    }
                    self.sort_tasks();
                    self.save_to_file();
                }
                if ui.add_sized([120.0, 24.0], egui::Button::new("完了済みを削除")).clicked() {
                    self.items.retain(|item| !item.completed);
                    self.save_to_file();
                }
            });
            ui.add_space(8.0);

            // 新規タスク入力
            let response = ui.horizontal(|ui| {
                ui.text_edit_singleline(&mut self.new_item_text);

                if ui.button("📅").clicked() {
                    self.show_date_picker = !self.show_date_picker;
                }

                if self.new_item_date.is_some() {
                    ui.label(format!("期限: {}", TodoApp::format_date(self.new_item_date.unwrap())));
                }

                ui.button("追加").clicked()
            });

            // カレンダー表示
            if self.show_date_picker {
                egui::Window::new("日付選択")
                    .fixed_size([280.0, 300.0])
                    .collapsible(false)
                    .resizable(false)
                    .show(ctx, |ui| {
                        // 閉じるボタンを右上に配置
                        ui.horizontal(|ui| {
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::RIGHT), |ui| {
                                if ui.button("×").clicked() {
                                    self.show_date_picker = false;
                                }
                            });
                        });

                        ui.horizontal(|ui| {
                            if ui.button("◀").clicked() {
                                if self.selected_month == 1 {
                                    self.selected_month = 12;
                                    self.selected_year -= 1;
                                } else {
                                    self.selected_month -= 1;
                                }
                            }
                            ui.label(format!("{:04}年{:02}月", self.selected_year, self.selected_month));
                            if ui.button("▶").clicked() {
                                if self.selected_month == 12 {
                                    self.selected_month = 1;
                                    self.selected_year += 1;
                                } else {
                                    self.selected_month += 1;
                                }
                            }
                        });

                        ui.add_space(8.0);

                        // 曜日の表示
                        ui.horizontal(|ui| {
                            for day in ["日", "月", "火", "水", "木", "金", "土"] {
                                ui.add_sized([35.0, 20.0], egui::Label::new(day));
                            }
                        });

                        let days_in_month = chrono::NaiveDate::from_ymd_opt(
                            self.selected_year,
                            self.selected_month,
                            1
                        ).unwrap().with_day(1).unwrap();

                        let first_weekday = days_in_month.weekday().num_days_from_sunday();
                        let total_days = days_in_month.with_month(self.selected_month + 1)
                            .unwrap_or_else(|| days_in_month.with_month(1).unwrap().with_year(self.selected_year + 1).unwrap())
                            .signed_duration_since(days_in_month).num_days();

                        let mut day = 1;
                        let mut week = 0;
                        while day <= total_days {
                            ui.horizontal(|ui| {
                                for weekday in 0..7 {
                                    if week == 0 && weekday < first_weekday as i64 {
                                        ui.add_sized([35.0, 30.0], egui::Label::new(" "));
                                    } else if day <= total_days {
                                        let btn = ui.add_sized(
                                            [35.0, 30.0],
                                            egui::Button::new(format!("{:2}", day))
                                        );
                                        if btn.clicked() {
                                            let date = chrono::NaiveDate::from_ymd_opt(
                                                self.selected_year,
                                                self.selected_month,
                                                day as u32
                                            ).unwrap();
                                            let datetime = date.and_hms_opt(23, 59, 59).unwrap();
                                            self.new_item_date = Some(datetime.and_utc().timestamp());
                                            self.show_date_picker = false;
                                        }
                                        day += 1;
                                    }
                                }
                            });
                            week += 1;
                        }
                    });
            }

            if response.inner {
                if !self.new_item_text.is_empty() {
                    self.items.push(TodoItem {
                        text: self.new_item_text.clone(),
                        completed: false,
                        due_date: self.new_item_date,
                    });
                    self.new_item_text.clear();
                    self.new_item_date = None;
                    self.sort_tasks();
                    self.save_to_file();
                }
            }

            ui.add_space(8.0);

            let mut changed = false;
            let mut to_remove = None;

            // 事前に各タスクの色を計算
            let task_colors: Vec<_> = self.items.iter()
                .map(|item| self.get_task_color(item.due_date))
                .collect();

            for ((index, item), color) in self.items.iter_mut().enumerate().zip(task_colors.iter()) {
                ui.horizontal(|ui| {
                    if ui.checkbox(&mut item.completed, "").changed() {
                        changed = true;
                    }
                    let mut label = egui::RichText::new(&item.text).color(*color);
                    if item.completed {
                        label = label.strikethrough();
                    }
                    ui.add_sized([300.0, 20.0], egui::Label::new(label));
                    if let Some(due_date) = item.due_date {
                        let date_label = egui::RichText::new(format!("期限: {}", TodoApp::format_date(due_date))).color(*color);
                        ui.add_sized([100.0, 20.0], egui::Label::new(date_label));
                    }
                    if ui.add_sized([50.0, 24.0], egui::Button::new("削除")).clicked() {
                        to_remove = Some(index);
                    }
                });
            }

            if changed {
                self.sort_tasks();
                self.save_to_file();
            }

            if let Some(index) = to_remove {
                self.items.remove(index);
                self.sort_tasks();
                self.save_to_file();
            }
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([480.0, 640.0]),
        ..Default::default()
    };

    eframe::run_native(
        "Todoアプリ",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();

            // OSに応じたフォント設定
            #[cfg(target_os = "windows")]
            {
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    egui::FontData::from_static(include_bytes!("C:\\Windows\\Fonts\\msgothic.ttc")).into(),
                );
            }

            #[cfg(target_os = "macos")]
            {
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    egui::FontData::from_static(include_bytes!("/System/Library/Fonts/ヒラギノ角ゴシック W3.ttc")).into(),
                );
            }

            #[cfg(target_os = "linux")]
            {
                // Linuxの場合は通常、Noto Sans CJK JPなどが/usr/share/fontsにある
                fonts.font_data.insert(
                    "system_font".to_owned(),
                    egui::FontData::from_static(include_bytes!("/usr/share/fonts/noto-cjk/NotoSansCJK-Regular.ttc")).into(),
                );
            }

            // フォントファミリーの設定
            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Proportional) {
                family.insert(0, "system_font".to_owned());
            }

            if let Some(family) = fonts.families.get_mut(&egui::FontFamily::Monospace) {
                family.push("system_font".to_owned());
            }

            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(TodoApp::new()))
        }),
    )
}