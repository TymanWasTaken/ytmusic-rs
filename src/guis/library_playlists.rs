use std::sync::Arc;

use iced::{Column, Text, Row, Command, futures::lock::Mutex, Element};
use reqwest::Method;
use ytmusic_api::{
    YtMusicClient,
    structs::{RequestBody, RequestContext},
    parsing::{ResponseType, YtMusicResponse}
};
use crate::Message;

pub fn render<'a>(gui: &crate::MusicGui) -> Vec<Element<'a, Message>> {
    let mut elements: Vec<Element<Message>> = vec![];
    match &gui.playlists {
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

pub fn load(gui: &crate::MusicGui, client: Arc<Mutex<YtMusicClient>>) -> Command<Message> {
    match &gui.playlists {
        Some(_) => Command::none(),
        None => Command::perform(async move {
            let client = client.lock().await;
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
        })
    }
}