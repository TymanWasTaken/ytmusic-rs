use std::sync::Arc;

use iced::{
    executor, Application, Column,
    Command, Container, Element, Length, Settings, Text, Clipboard, futures::lock::Mutex, VerticalAlignment, Row,
};
use reqwest::Method;
use ytmusic_api::{structs::{Playlist, RequestBody, RequestContext}, YtMusicClient, parsing::{YtMusicResponse, ResponseType}};

pub fn main() -> iced::Result {
    MusicGui::run(Settings::default())
}

#[allow(dead_code)]
struct MusicGui {
    state: State,
    playlists: Option<Vec<Playlist>>,
    client: Arc<Mutex<YtMusicClient>>
}

enum State {
    NotLoading,
    Loaded
}

#[derive(Debug)]
enum Message {
    LoadPlaylists(Vec<Playlist>)
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
                state: State::NotLoading,
                playlists: None,
                client: client.clone()
            },
            Command::perform(async move {
                let client = client_clone.lock().await;
                client.set_headers().await;
                let playlists = client.endpoint("browse").make_request(
                    Method::POST,
                    RequestBody {
                        browseId: "FEmusic_liked_playlists".to_string(),
                        context: RequestContext::new()
                    }.as_body()
                ).await.unwrap();
                playlists.parse(ResponseType::LibraryPlaylists).await
            }, |playlists| {
                Message::LoadPlaylists(playlists)
            }),
        )
    }

    fn title(&self) -> String {
        String::from("Youtube music")
    }

    fn update(&mut self, message: Message, _clipboard: &mut Clipboard) -> Command<Message> {
        match message {
            Message::LoadPlaylists(playlists) => {
                self.state = State::Loaded;
                self.playlists = Some(playlists);
            }
        }

        Command::none()
    }

    // fn subscription(&self) -> Subscription<Message> {
    //     match self.state {
    //         State::Loading { .. } => {
    //             Subscription::from(async {

    //             })
    //         },
    //         _ => Subscription::none()
    //     }
    // }

    fn view(&mut self) -> Element<Message> {
        let text = Text::new("Youtube Music")
            .size(40).vertical_alignment(VerticalAlignment::Top);

        let mut content = Column::new()
            .align_items(iced::Align::Center)
            .spacing(20)
            .push(text);

        match &self.playlists {
            Some(playlists) => {
                let mut playlists_column = Column::new()
                    .align_items(iced::Align::Center)
                    .spacing(10);
                
                for (i, playlist) in playlists.iter().enumerate() {
                    playlists_column = playlists_column.push(
                        Row::new()
                            .spacing(10)
                            .push(Text::new(format!("{}.", i + 1)))
                            .push(Text::new(playlist.title.as_str()))
                    );
                }
                content = content.push(playlists_column);
            },
            None => {
                content = content.push(
                    Text::new("Loading...").size(20)
                );
            }
        }

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}

mod style {
    use iced::{button, Background, Color, Vector};

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
                border_radius: 12.0,
                shadow_offset: Vector::new(1.0, 1.0),
                text_color: Color::WHITE,
                ..button::Style::default()
            }
        }
    }
}