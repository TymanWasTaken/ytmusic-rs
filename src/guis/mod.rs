mod library_playlists;
mod home;

use iced::{Command, Element};
pub use library_playlists::LibraryPlaylistsPage;
pub use home::HomePage;

use crate::Message;

pub enum PageInstance {
    Home(HomePage),
    LibraryPlaylists(LibraryPlaylistsPage)
}

impl PageInstance {
    pub fn load(&self) -> Option<Command<Message>> {
        match self {
            PageInstance::Home(home) => None,
            PageInstance::LibraryPlaylists(playlists) => Some(playlists.load())
        }
    }

    pub fn render(&self) -> Vec<Element<Message>> {
        match self {
            PageInstance::Home(home) => home.render(),
            PageInstance::LibraryPlaylists(playlists) => playlists.render()
        }
    }
}