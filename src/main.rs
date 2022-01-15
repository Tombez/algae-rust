mod vec2;

pub mod algae {
    use std::{time};
    use std::time::Instant;
    use simple_websockets::{Event, EventHub, Responder, Message};
    use std::collections::HashMap;
    use crate::vec2::Vec2;

    fn string_at(bytes: &[u8], mut i: usize) -> &str {
        let len = bytes.len();
        while i < len {
            if bytes[i] == 0 { break; }
            i += 1;
        }
        let result = std::str::from_utf8(&bytes[1..i]);
        match result {
            Ok(s) => s,
            _ => ""
        }
    }
    struct Player {
        name: String,
        mouse: Vec2<f32>
    }
    struct Client {
        ws_client: Responder,
        player: Option<Player>
    }

    impl Client {
        pub fn new(ws_client: Responder) -> Client {
            Client { ws_client, player: None }
        }
    }

    pub struct Server {
        ws_server: EventHub,
        clients: HashMap<u64, Client>,
        last_tick_instant: Instant,
        last_id: u32
    }
    impl Default for Server {
        fn default() -> Server {
            Self::new()
        }
    }
    impl Server {
        pub fn new() -> Server {
            let ws_server = simple_websockets::launch(8080)
            .expect("failed to listen on port 8080");
            let last_tick_instant = time::Instant::now();
            Server {ws_server, clients: HashMap::new(), last_tick_instant, last_id: 0}
        }
        pub fn start(&mut self) {
            loop { self.game_loop(); }
        }
        fn game_loop(&mut self) {
            let inter_tick_duration = self.last_tick_instant.elapsed();
            if inter_tick_duration < std::time::Duration::new(0, 1_000_000_000 / 60) {
                //println!("spinning because duration is not large enough: {:?}, min: {:?}", inter_tick_duration, std::time::Duration::new(0, 1_000_000_000 / 60));
                return;
            }
            let frame_start = time::Instant::now();

            loop {
                match self.ws_server.next_event() {
                    None => { break; },
                    Some(event) => {
                        match event {
                            Event::Connect(client_id, responder) => {
                                println!("A client connected with id #{}", client_id);
                                let client = Client::new(responder);
                                let msg = vec![2u8, 60u8, 200u8];
                                client.ws_client.send(Message::Binary(msg));
                                self.clients.insert(client_id, client);
                            },
                            Event::Disconnect(client_id) => {
                                println!("Client #{} disconnected.", client_id);
                                self.clients.remove(&client_id);
                            },
                            Event::Message(client_id, message) => {
                                // println!("Received a message from client #{}: {:?}", client_id, message);
                                let client = self.clients.get_mut(&client_id).unwrap();
                                if let Message::Binary(bytes) = message {
                                    let id = bytes[0];
                                    match id {
                                        254 => {

                                        }
                                        255 => {

                                        }
                                        0 => {
                                            let name = string_at(&bytes, 1);
                                            println!("play msg name: {}", name);
                                            let mouse = Vec2 {x: 0f32, y: 0f32};
                                            let player = Player {name: name.to_string(), mouse};
                                            client.player = Some(player);
                                        }
                                        16 => {
                                            //todo! handle mouse input
                                        },
                                        _ => {
                                            println!("unknown message id: {}, msg: {:?}", id, bytes);
                                        }
                                    }
                                }
                            },
                        }
                    }
                }
            }

            // todo!("game logic");
            // todo!("send updates to clients");
            let _tick_duration = frame_start.elapsed();
            // println!("finished tick with duration: {:?}", tick_duration);
            self.last_tick_instant = frame_start;
        }
    }
}

fn main() {
    let mut server = algae::Server::new();
    server.start();
}
