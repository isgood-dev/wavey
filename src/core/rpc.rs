use std::thread;
use std::sync::mpsc;

use super::format;

use discord_presence::Client;

#[derive(Debug, Clone)]
pub enum RpcEvent {
    Set(String, String),
    SetProgress(String, String, String),
    Hide,

}

pub fn start_receiver(reciever: mpsc::Receiver<RpcEvent>) {
    // Change the client number if you'd like, however it will look for an asset named "icon"
    // which must be present in the Discord client configuration.
    let mut client = Client::new(1244949516965118014);

    client.start();

    thread::spawn(move || {
        loop {
            if let Ok(command) = reciever.try_recv() {
                process_rpc_command(command, &mut client);
            }

            thread::sleep(std::time::Duration::from_millis(100));
        }
    });
}

fn process_rpc_command(command: RpcEvent, client: &mut Client) {
    match command {
        RpcEvent::Set(display_name, duration) => {
            let duration_u64: u64 = duration.parse().expect("Failed to parse duration");
            let _ = client.set_activity(|a| {
                a.state(format!("0:00 / {}", format::format_duration(duration_u64)).as_str())
                .assets(|ass| {
                    ass.large_image("icon")
                        .large_text("wavey by ISgood Development")
                        .small_text("wavey by ISgood Development")
                })
                .details(display_name)
            });
        }

        RpcEvent::SetProgress(display_name, progress, duration) => {
            let duration_u64: u64 = duration.parse().expect("Failed to parse duration");
            let progress_u64: u64 = progress.parse().expect("Failed to parse progress");

            let _ = client.set_activity(|a: discord_presence::models::Activity| {
                a.state(format!("{} / {}", format::format_duration(progress_u64), format::format_duration(duration_u64)))
                .assets(|ass| {
                    ass.large_image("icon")
                        .large_text("wavey by ISgood Development")
                        .small_text("wavey by ISgood Development")
                })
                .details(display_name)
            });
        }

        RpcEvent::Hide => {
            let _ = client.clear_activity();
        }
    }
}
