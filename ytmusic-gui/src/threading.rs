use tokio::runtime::Builder;
use tokio::sync::mpsc;
use ytmusic_api::structs::{Playlist, RequestContext, RequestBody};
use ytmusic_api::{
    parsing::{YtMusicResponse, ResponseType},
    YtMusicClient
};
use reqwest::Method;

pub struct Task {
    name: String,
    data: Vec<Playlist>
}

pub struct TaskSpawner {
    callback: Box<dyn Fn() -> ()>
};

impl TaskSpawner {
    pub fn new(callback: &dyn Fn() -> ()) -> TaskSpawner {
        // Set up a channel for communicating.
        let (send, mut recv) = mpsc::channel(16);

        // Build the runtime for the new thread.
        //
        // The runtime is created before spawning the thread
        // to more cleanly forward errors if the `unwrap()`
        // panics.
        let rt = Builder::new_current_thread()
            .enable_all()
            .build()
            .unwrap();

        std::thread::spawn(move || {
            rt.block_on(async move {
                let client = YtMusicClient::new("headers.txt");
                client.set_headers().await;

                let result = client
                    .endpoint("browse")
                    .make_request(
                        Method::POST,
                        RequestBody {
                            browseId: "FEmusic_liked_playlists".to_string(),
                            context: RequestContext::new(),
                        }
                        .as_body(),
                    )
                    .await
                    .unwrap()
                    .parse(ResponseType::LibraryPlaylists)
                    .await;
                send.send(Task { name: "loaded_playlists".to_string(), data: result }).await;
            });
        });

        TaskSpawner {
            callback: Box::new(callback.clone())
        }
    }

    pub fn spawn_task(&self, task: Task) {
        match self.spawn.blocking_send(task) {
            Ok(()) => {},
            Err(_) => panic!("The shared runtime has shut down."),
        }
    }
}