mod vec2;
mod protocol;

pub mod algae {
    const SERVER_PORT: u16 = 8080;
    const CELL_START_R: f32 = 5.0;

    use std::rc::Rc;
    use std::{time};
    use std::time::Instant;
    use simple_websockets::{Event, EventHub, Responder, Message};
    use std::collections::HashMap;
    use crate::vec2::Vec2;
    use crate::protocol::{ServerOpcode, ClientOpcode};

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

    struct Cell {
        id: u32,
        pos: Vec2<f32>,
        r: f32,
    }

    struct Player {
        name: String,
        mouse: Vec2<f32>,
        cell_ids: Vec<u32>
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
        clients: HashMap<u64, Box<Client>>,
        last_tick_instant: Instant,
        last_id: u32,
        cells_by_id: HashMap<u32, Box<Cell>>,
    }
    impl Default for Server {
        fn default() -> Server {
            Self::new()
        }
    }
    impl Server {
        pub fn new() -> Server {
            let failed_str = std::format!("failed to listen on port {}", SERVER_PORT);
            let ws_server = simple_websockets::launch(SERVER_PORT)
            .expect(&failed_str);
            let clients = HashMap::new();
            let last_tick_instant = time::Instant::now();
            let cells_by_id = HashMap::new();
            Server {ws_server, clients, last_tick_instant, last_id: 0, cells_by_id}
        }
        pub fn start(mut self) {
            loop { self = self.game_loop(); }
        }
        fn game_loop(mut self) -> Self {
            let inter_tick_duration = self.last_tick_instant.elapsed();
            if inter_tick_duration < std::time::Duration::new(0, 1_000_000_000 / 60) {
                //println!("spinning because duration is not large enough: {:?}, min: {:?}", inter_tick_duration, std::time::Duration::new(0, 1_000_000_000 / 60));
                return self;
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
                                self.clients.insert(client_id, Box::new(client));
                            },
                            Event::Disconnect(client_id) => {
                                println!("Client #{} disconnected.", client_id);
                                self.clients.remove(&client_id);
                            },
                            Event::Message(client_id, message) => {
                                // println!("Received a message from client #{}: {:?}", client_id, message);
                                if let Message::Binary(bytes) = message {
                                    let mut client = self.clients.remove(&client_id).unwrap();
                                    let opcode = ClientOpcode::from(bytes[0]);
                                    match opcode {
                                        ClientOpcode::VersionHandshake => {
                                            // todo
                                        }
                                        ClientOpcode::Handshake2 => {
                                            // ignore
                                        }
                                        ClientOpcode::Play => {
                                            let name = string_at(&bytes, 1);
                                            println!("play msg name: {}", name);
                                            let mouse = Vec2 {x: 0f32, y: 0f32};
                                            let cell_ids = vec![];
                                            let mut player = Player {name: name.to_string(), mouse, cell_ids};

                                            self.last_id += 1;
                                            let id = self.last_id;
                                            let pos = Vec2{x: 0f32, y: 0f32};
                                            let cell = Cell{id, pos, r: CELL_START_R};
                                            player.cell_ids.push(cell.id);

                                            client.player = Some(player);
                                        }
                                        ClientOpcode::MousePos => {
                                            //todo! handle mouse input
                                        },
                                        _ => {
                                            println!("unknown opcode: {:?}, msg: {:?}", opcode, bytes);
                                        }
                                    }
                                    self.clients.insert(client_id, client);
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
            self
        }
        // fn spawnPlayer(mut self, client: &mut Client, name: &str) -> Self {

        //     self
        // }
    }
}

fn main() {
    let mut server = algae::Server::new();
    server.start();
}
