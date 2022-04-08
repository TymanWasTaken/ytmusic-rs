use std::sync::Arc;

use iced::{Column, Text, Row, Command, futures::lock::Mutex};
use reqwest::Method;
use ytmusic_api::{
    YtMusicClient,
    structs::{RequestBody, RequestContext},
    parsing::{ResponseType, YtMusicResponse}
};
use crate::Message;

pub fn render<'a>(gui: &crate::MusicGui, mut content: Column<'a, Message>) -> Column<'a, Message> {
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
            content = content.push(playlists_column);
        },
        None => {
            content = content.push(
                Text::new("Loading...").size(20)
            );
        }
    }
    content
}

pub fn load(client: Arc<Mutex<YtMusicClient>>) -> Command<Message> {
    Command::perform(async move {
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