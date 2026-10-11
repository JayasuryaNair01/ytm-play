use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::io::{self, Write};
use std::process::Command;

#[derive(Deserialize, Debug, Clone)]
struct Song {
    title: String,
    artist: String,
    video_id: String,
}

#[derive(Serialize, Debug)]
struct InnerClient {
    #[serde(rename = "clientName")]
    client_name: String,
    #[serde(rename = "clientVersion")]
    client_version: String,
    hl: String,
    gl: String,
}

#[derive(Serialize, Debug)]
struct ApiContext {
    client: InnerClient,
}

#[derive(Serialize, Debug)]
struct SearchPayload {
    context: ApiContext,
    query: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== YTM-PLAY INITIALIZED ===");

    let client = Client::new();
    let target_url = "https://music.youtube.com/youtubei/v1/search";

    // Continuous Loop: Keeps you in the player experience until you exit
    loop {
        println!("\n----------------------------------------");
        print!("Enter song or artist to search (or type 'exit' to quit): ");
        io::stdout().flush()?;

        let mut query = String::new();
        io::stdin().read_line(&mut query)?;
        let query = query.trim();

        if query.is_empty() {
            continue;
        }

        if query.eq_ignore_ascii_case("exit") {
            println!("[*] Exiting YTM-PLAY. Goodbye!");
            break;
        }

        println!("[+] Searching for: \"{}\"", query);

        let payload = SearchPayload {
            context: ApiContext {
                client: InnerClient {
                    client_name: "WEB_REMIX".to_string(),
                    client_version: "1.20231005.01.00".to_string(),
                    hl: "en".to_string(),
                    gl: "US".to_string(),
                },
            },
            query: query.to_string(),
        };

        let response = match client
            .post(target_url)
            .query(&[("key", "AIzaSyC9XL3ZjWddXya6X74dJoCTL-WEYFDNX30")])
            .json(&payload)
            .send()
            .await
        {
            Ok(resp) => resp,
            Err(e) => {
                println!("[-] Network request failed: {}", e);
                continue;
            }
        };

        if response.status().is_success() {
            let body = response.text().await?;

            match serde_json::from_str::<serde_json::Value>(&body) {
                Ok(json) => {
                    let mut song_list = Vec::new();

                    let sections = json
                        .get("contents")
                        .and_then(|c| c.get("tabbedSearchResultsRenderer"))
                        .and_then(|t| t.get("tabs"))
                        .and_then(|tabs| tabs.get(0))
                        .and_then(|tab| tab.get("tabRenderer"))
                        .and_then(|tr| tr.get("content"))
                        .and_then(|content| content.get("sectionListRenderer"))
                        .and_then(|sl| sl.get("contents"))
                        .and_then(|s| s.as_array());

                    if let Some(sec_list) = sections {
                        for section in sec_list {
                            // Check standard shelves, carousels (artist top tracks/albums), and item sections
                            let contents_opt = section
                                .get("musicShelfRenderer")
                                .and_then(|s| s.get("contents"))
                                .and_then(|c| c.as_array())
                                .or_else(|| {
                                    section
                                        .get("musicCarouselShelfRenderer")
                                        .and_then(|s| s.get("contents"))
                                        .and_then(|c| c.as_array())
                                })
                                .or_else(|| {
                                    section
                                        .get("itemSectionRenderer")
                                        .and_then(|s| s.get("contents"))
                                        .and_then(|c| c.as_array())
                                });

                            if let Some(contents) = contents_opt {
                                for track in contents {
                                    let item_opt = track
                                        .get("musicResponsiveListItemRenderer")
                                        .or_else(|| {
                                            track.get("musicTwoRowItemRenderer") // Handles albums/singles grid items
                                        });

                                    if let Some(item) = item_opt {
                                        let title = item
                                            .get("flexColumns")
                                            .and_then(|cols| cols.get(0))
                                            .and_then(|col| {
                                                col.get("musicResponsiveListItemFlexColumnRenderer")
                                            })
                                            .and_then(|col_r| col_r.get("text"))
                                            .and_then(|text| text.get("runs"))
                                            .and_then(|runs| runs.get(0))
                                            .and_then(|run| run.get("text"))
                                            .and_then(|t| t.as_str())
                                            .or_else(|| {
                                                // Fallback for grid items (two-row renderer)
                                                item.get("title")
                                                    .and_then(|t| t.get("runs"))
                                                    .and_then(|runs| runs.get(0))
                                                    .and_then(|run| run.get("text"))
                                                    .and_then(|t| t.as_str())
                                            });

                                        let video_id = item
                                            .get("navigationEndpoint")
                                            .and_then(|ne| ne.get("watchEndpoint"))
                                            .and_then(|we| we.get("videoId"))
                                            .and_then(|v| v.as_str())
                                            .or_else(|| {
                                                item.get("playlistItemData")
                                                    .and_then(|pid| pid.get("videoId"))
                                                    .and_then(|v| v.as_str())
                                            });

                                        if let (Some(t), Some(id)) = (title, video_id) {
                                            if !song_list.iter().any(|s: &Song| s.video_id == id) {
                                                song_list.push(Song {
                                                    title: t.to_string(),
                                                    artist: query.to_string(),
                                                    video_id: id.to_string(),
                                                });
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if !song_list.is_empty() {
                        println!("\n[+] Found results:\n");
                        for (i, song) in song_list.iter().take(10).enumerate() {
                            println!("  {}. {}", i + 1, song.title);
                        }

                        print!("\nEnter track number to play (or 0 to search again): ");
                        io::stdout().flush()?;

                        let mut choice = String::new();
                        io::stdin().read_line(&mut choice)?;

                        if let Ok(index) = choice.trim().parse::<usize>() {
                            if index == 0 {
                                continue;
                            }
                            if index > 0 && index <= song_list.len() {
                                let selected = &song_list[index - 1];
                                let watch_url = format!(
                                    "https://music.youtube.com/watch?v={}",
                                    selected.video_id
                                );

                                println!(
                                    "\n[*] Playing: {} (Press 'q' in player to skip/stop, or Ctrl+C to exit)",
                                    selected.title
                                );

                                // Spawns `mpv`. While playing, mpv takes standard input (so pressing space pauses, q quits back to loop).
                                let _status = Command::new("mpv")
                                    .arg("--no-video")
                                    .arg(&watch_url)
                                    .status();

                                println!("\n[*] Track finished. Returning to search menu...");
                            } else {
                                println!("[-] Invalid selection number.");
                            }
                        }
                    } else {
                        println!("[-] No playable tracks found for this query.");
                    }
                }
                Err(e) => {
                    println!("[-] Failed to parse JSON: {}", e);
                }
            }
        } else {
            println!("[-] Request failed with status: {}", response.status());
        }
    }

    Ok(())
}
