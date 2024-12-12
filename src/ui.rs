use egui::Context;

pub struct App {
    pub current_screen: Screen, // змінна для управління поточним екраном
    pub tasks: Vec<crate::task::Task>,
    pub new_title: String,
    pub new_description: String,
    pub selected_task: Option<usize>, // індекс вибраного завдання
    pub edit_task: crate::task::Task, // змінна для редагуємого завдання
}


pub enum Screen {
    TaskList,
    TaskEdit,
}

impl App {
    pub fn new() -> Self {
        let tasks = crate::storage::load_tasks().unwrap_or_else(|_| Vec::new());
        App {
            current_screen: Screen::TaskList,
            tasks,
            new_title: String::new(),
            new_description: String::new(),
            selected_task: None,
            edit_task: crate::task::Task::new("", ""),
        }
    }
}

// реалізація трейту eframe::App
impl eframe::App for App {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::TaskList => self.show_task_list(ui),
                Screen::TaskEdit => self.show_task_edit(ui),
            }
        });
    }
}

impl App {
    pub fn show_task_list(&mut self, ui: &mut egui::Ui) {
        ui.heading("Додаток для списка справ");

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Заголовок:");
                    ui.text_edit_singleline(&mut self.new_title);
                });
                ui.horizontal(|ui| {
                    ui.label("Опис:");
                    ui.text_edit_multiline(&mut self.new_description);
                });
            });
            if ui.button("Додати").clicked() {
                self.tasks.push(crate::task::Task::new(&self.new_title, &self.new_description));
                self.new_title.clear();
                self.new_description.clear();
            }
        });

        ui.separator();

        ui.label("Список справ:");

        let mut tasks_to_remove = Vec::new(); // зберігаємо ідентифікатори завдань для видалення
        for (index, task) in self.tasks.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.checkbox(&mut task.completed, "");
                ui.label(&task.title);
                if ui.button("Редагувати").clicked() {
                    self.selected_task = Some(index);
                    self.edit_task = task.clone();
                    self.current_screen = Screen::TaskEdit;
                }
                if ui.button("Видалити").clicked() {
                    tasks_to_remove.push(index);
                }
            });
        }

        // видалення завдань після циклу
        for index in tasks_to_remove.into_iter().rev() {
            self.tasks.remove(index);
        }

        if ui.button("Зберегти").clicked() {
            crate::storage::save_tasks(&self.tasks).expect("Не вдалося зберегти завдання");
        }

        ui.separator();

        if ui.button("Зберегти у файл").clicked() {
            crate::storage::save_tasks_to(&self.tasks).expect("Не вдалося зберегти завдання");
        }

        if ui.button("Завантажити з файлу").clicked() {
            self.tasks = crate::storage::load_tasks_from().unwrap_or_else(|_| Vec::new());
        }
    }


    pub fn show_task_edit(&mut self, ui: &mut egui::Ui) {
        ui.heading("Редагування завдання");

        ui.horizontal(|ui| {
            ui.label("Статус:");
            ui.checkbox(&mut self.edit_task.completed, "");
        });
        ui.horizontal(|ui| {
            ui.label("Заголовок:");
            ui.text_edit_singleline(&mut self.edit_task.title);
        });
        ui.horizontal(|ui| {
            ui.label("Опис:");
            ui.text_edit_multiline(&mut self.edit_task.description);
        });

        if ui.button("Зберегти").clicked() {
            if let Some(index) = self.selected_task {
                self.tasks[index] = self.edit_task.clone();
            }
            self.current_screen = Screen::TaskList;
        }

        if ui.button("Скасувати").clicked() {
            self.current_screen = Screen::TaskList;
        }
    }
}
