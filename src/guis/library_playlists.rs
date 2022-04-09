use std::sync::Arc;

use iced::{Column, Text, Row, Command, futures::lock::Mutex, Element};
use reqwest::Method;
use ytmusic_api::{
    YtMusicClient,
    structs::{RequestBody, RequestContext},
    parsing::{ResponseType, YtMusicResponse}
};
use crate::{Message, MusicGui};

pub struct LibraryPlaylistsPage {
    gui: Option<Arc<MusicGui>>,
    client: Arc<Mutex<YtMusicClient>>
}

impl LibraryPlaylistsPage {
    pub fn new(client: Arc<Mutex<YtMusicClient>>) -> Self {
        Self {
            gui: None,
            client
        }
    }

    pub fn set_gui(&mut self, gui: Arc<MusicGui>) {
        self.gui = Some(gui);
    }

    pub fn render<'a>(&self) -> Vec<Element<'a, Message>> {
        let mut elements: Vec<Element<Message>> = vec![];
        match &self.gui.unwrap().playlists {
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
                elements.push(playlists_column.into());
            },
            None => {
                elements.push(
                    Text::new("Loading...").size(20).into()
                );
            }
        }
        elements
    }
    
    pub fn load(&self) -> Command<Message> {
        let mut commands: Vec<Command<Message>> = vec![];
        // Load playlists if not already loaded
        if let None = &self.gui.unwrap().playlists {
            commands.push(Command::perform(async {
                let client = self.client.lock().await;
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
            }))
        }
    
        // Make commands to download each image
    
        Command::batch(commands)
    }
}