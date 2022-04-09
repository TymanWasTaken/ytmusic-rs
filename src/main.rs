mod guis;

use std::sync::Arc;
use iced::{
    executor, Application, Column,
    Command, Container, Element, Length, 
    Settings, Text, Clipboard, futures::lock::Mutex, 
    button, Row, HorizontalAlignment, Button
};
use ytmusic_api::{structs::Playlist, YtMusicClient};

pub fn main() -> iced::Result {
    MusicGui::run(Settings::default())
}

pub struct Page {
    pub name: String,
    pub state: State,
    pub nav_state: button::State,
    pub instance: guis::PageInstance
}

#[allow(dead_code)]
pub struct MusicGui {
    state: State,
    pages: Vec<Page>,
    playlists: Option<Vec<Playlist>>,
    client: Arc<Mutex<YtMusicClient>>
}

#[derive(Debug, Clone)]
pub enum State {
    Home,
    Playlists
}

#[derive(Debug, Clone)]
pub enum Message {
    LoadPlaylists(Vec<Playlist>),
    HeadersSet,
    Navigate(State)
}

impl Application for MusicGui {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (MusicGui, Command<Message>) {
        let client = Arc::new(Mutex::new(YtMusicClient::new("headers.txt")));
        let client_clone = client.clone();
        (
            MusicGui {
                state: State::Home,
                playlists: None,
                client: client.clone(),
                pages: vec![
                    Page {
                        name: "Home".to_string(),
                        state: State::Home,
                        nav_state: button::State::new(),
                        instance: guis::PageInstance::Home(guis::HomePage::new())
                    },
                    Page {
                        name: "Playlists".to_string(),
                        state: State::Playlists,
                        nav_state: button::State::new(),
                        instance: guis::PageInstance::LibraryPlaylists(guis::LibraryPlaylistsPage::new(client.clone()))
                    }
                ]
            },
            Command::perform(async move {
                let client = client_clone.lock().await;
                client.set_headers().await;
            }, |_| {
                Message::HeadersSet
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Youtube music")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        let mut command = Command::none();
        match message {
            Message::HeadersSet => (),
            Message::LoadPlaylists(playlists) => {
                self.playlists = Some(playlists);
            },
            Message::Navigate(state) => {
                match &state {
                    State::Home => (),
                    State::Playlists => command = self.pages[1].instance.load().unwrap_or(Command::none())
                }
                self.state = state;
            }
        }

        command
    }

    fn view(&mut self) -> Element<Message> {
        let mut elements: Vec<Element<Message>> = match &self.state {
            State::Home => self.pages[0].instance.render(),
            State::Playlists => self.pages[1].instance.render()
        };

        let mut top_row: Row<Message> = Row::new()
            .align_items(iced::Align::Center)
            .width(Length::Fill)
            .padding(10)
            .spacing(20);
        let text = Text::new("YT Music")
            .horizontal_alignment(HorizontalAlignment::Left);
        top_row = top_row.push(text);

        let mut btn_row = Row::new()
            .spacing(10)
            .width(Length::Fill);
        for page in &mut self.pages {
            let btn = Button::new(
                &mut page.nav_state,
                Text::new(format!("Go to {}", page.name))
            )
                .on_press(Message::Navigate(page.state.clone()))
                .style(crate::style::Button::Primary);
            btn_row = btn_row.push(
                btn
            );
        }
        let top_row = top_row.push(btn_row);
        elements.insert(0, top_row.into());

        let content = Column::with_children(elements)
            .align_items(iced::Align::Center)
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(20);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .style(style::Container::Dark)
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector, container};

    #[allow(dead_code)]
    pub enum Button {
        Primary,
        Secondary,
        Destructive,
    }

    impl button::StyleSheet for Button {
        fn active(&self) -> button::Style {
            button::Style {
                background: Some(Background::Color(match self {
                    Button::Primary => Color::from_rgb(0.11, 0.42, 0.87),
                    Button::Secondary => Color::from_rgb(0.5, 0.5, 0.5),
                    Button::Destructive => Color::from_rgb(0.8, 0.2, 0.2),
                })),
                border_radius: 5.0,
                shadow_offset: Vector::new(0.5, 0.5),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }

    pub enum Container {
        Dark
    }

    impl container::StyleSheet for Container {
        fn style(&self) -> container::Style {
            container::Style {
                background: Some(Background::Color(Color::from_rgb(0.1, 0.1, 0.1))),
                text_color: Some(Color::WHITE),
                ..container::Style::default()
            }
        }
    }
}