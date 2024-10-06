use std::{
    fs::{self, File},
    path::PathBuf,
};

use game_core::crops::data::CropDefinition;
use iced::{
    widget::{button, column, container, keyed_column, row, scrollable, text},
    Element, Theme,
};
use rfd::FileDialog;

fn main() -> iced::Result {
    println!("Running tool");
    iced::application("Crops Editor", App::update, App::view)
        .theme(theme)
        .run()
}

fn theme(_: &App) -> Theme {
    Theme::CatppuccinMacchiato
}

struct App {
    screen: Screen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            screen: Default::default(),
        }
    }
}

#[derive(Default, Debug, Clone)]
enum Screen {
    #[default]
    Start,
    Open,
    Load(PathBuf),
    Edit {
        file: PathBuf,
        data: CropDefinition,
    },
}

#[derive(Debug, Clone)]
enum Messages {
    ChangeScreen(Screen),
    AddStage,
    RemoveStage(usize),
    MoveStage { index: usize, delta: i32 },
}
impl Screen {
    fn update(&self, _: Messages) {}

    fn view(&self) -> Element<Messages> {
        match self {
            Screen::Start => container(
                row![
                    "Startup",
                    button("Open File").on_press(Messages::ChangeScreen(Screen::Open))
                ]
                .spacing(10),
            )
            .into(),
            Screen::Open => {
                // TODO convert whole wortkflow to use RFD instead of janky screen picker
                let file = FileDialog::new()
                    .add_filter("Ron", &["ron"])
                    .set_directory("assets/core/data/crops")
                    .pick_file();

                match fs::read_dir("assets/core/data/crops") {
                    Err(err) => container(
                        column![
                            text("Failed to access directory with error:"),
                            text(format!("Error: {err}")),
                            button(text("Return")).on_press(Messages::ChangeScreen(Screen::Start))
                        ]
                        .spacing(10),
                    ),
                    Ok(dir) => container(scrollable(
                        column![
                            text("Available Files:"),
                            keyed_column(
                                dir.filter_map(|d| {
                                    let Ok(entry) = d else {
                                        return None;
                                    };
                                    let Ok(t) = entry.file_type() else {
                                        return None;
                                    };
                                    if !t.is_file() {
                                        return None;
                                    }
                                    let path = entry.path();
                                    let Some(name) = path.file_name() else {
                                        return None;
                                    };
                                    Some(
                                        button(text(format!(
                                            "{}",
                                            name.to_str().unwrap_or_default()
                                        )))
                                        .on_press(Messages::ChangeScreen(Screen::Load(path)))
                                        .into(),
                                    )
                                })
                                .enumerate(),
                            )
                            .spacing(5)
                        ]
                        .spacing(10),
                    ))
                    .into(),
                }
            }
            Screen::Load(path_buf) => {
                let Ok(file) = File::open(path_buf) else {
                    return container(
                        button(text("Failed somehow"))
                            .on_press(Messages::ChangeScreen(Screen::Open)),
                    )
                    .into();
                };
                let Ok(data) = ron::de::from_reader(file) else {
                    return container(
                        button(text("Failed to deserialize"))
                            .on_press(Messages::ChangeScreen(Screen::Open)),
                    )
                    .into();
                };
                container(
                    column![
                        text(format!("file: {}", path_buf.display())),
                        row![
                            button(text("Load")).on_press(Messages::ChangeScreen(Screen::Edit {
                                file: path_buf.clone(),
                                data
                            })),
                            button(text("Cancel")).on_press(Messages::ChangeScreen(Screen::Open))
                        ]
                        .spacing(10)
                    ]
                    .spacing(10),
                )
            }
            Screen::Edit { file: _, data } => {
                container(text(format!("Under construction : {:#?}", data))).into()
            }
        }
        .padding(20)
        .center(300)
        .into()
    }
}

impl App {
    fn update(&mut self, message: Messages) {
        if let Messages::ChangeScreen(n_screen) = message {
            self.screen = n_screen;
        } else {
            self.screen.update(message);
        }
    }

    fn view(&self) -> Element<Messages> {
        self.screen.view()
    }
}
