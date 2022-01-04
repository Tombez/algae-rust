extern crate websocket;

pub mod algae {
    use std::{time};
    use std::time::Instant;
    use websocket::server::WsServer;
    use websocket::server::NoTlsAcceptor;
    use std::net::{TcpListener, TcpStream};
    use websocket::sync::{Server as SyncServer};
    use websocket::OwnedMessage;
    use std::net::SocketAddr;
    use websocket::sync::Client as WsClient;

    struct Player {

    }
    struct Client {
        ws_client: WsClient<TcpStream>,
        ip: SocketAddr,
        should_close: bool
        // receiver: Reader<TcpStream>,
        // sender: Writer<TcpStream>
        // player: Player
    }

    impl Client {
        pub fn new(ws_client: WsClient<TcpStream>, ip: SocketAddr) -> Client {
            Client {
                ws_client,
                ip,
                should_close: false
            }
        }
    }

    pub struct Server {
        ws_server: WsServer<NoTlsAcceptor, TcpListener>,
        clients: Vec<Client>,
        last_tick_instant: Instant,
    }
    impl Server {
        pub fn new() -> Server {
            let ws_server = SyncServer::bind("127.0.0.1:8080").unwrap();
            let last_tick_instant = time::Instant::now();
            let server = Server {ws_server, clients: vec![], last_tick_instant};
            server
        }
        pub fn start(&mut self) {
            loop { self.game_loop(); }
        }
        fn game_loop(&mut self) {
            let inter_tick_duration = self.last_tick_instant.elapsed();
            if inter_tick_duration < std::time::Duration::new(0, 1_000_000_000 / 60) {
                return;
            }
            let frame_start = time::Instant::now();
            self.clients.append(&mut Server::accept_new_connections(&mut self.ws_server));
            Server::handle_client_messages(&mut self.clients);
            // todo!("game logic");
            // todo!("send updates to clients");
            let tick_duration = frame_start.elapsed();
            println!("finished tick with duration: {:?}", tick_duration);
            self.last_tick_instant = frame_start;
        }
        fn accept_new_connections(ws_server: &mut WsServer<NoTlsAcceptor, TcpListener>) -> Vec<Client> {
            let mut clients: Vec<Client> = vec![];
            let filtered = ws_server.filter_map(Result::ok);
            for request in filtered {
                let ws_client = request.accept().unwrap();
                let ip = ws_client.peer_addr().unwrap();
                println!("Connection from ip: {}", ip);

                // let (receiver, sender) = ws_client.split().unwrap();


                clients.push(Client::new(ws_client, ip));
            }
            clients
        }
        fn handle_client_messages(clients: &mut Vec<Client>) {
            for client in clients {
                'outer: for result in client.ws_client.incoming_messages() {
                    match result {
                        Ok(message) => {
                            println!("debug a");
                            match message {
                                OwnedMessage::Binary(data) => {
                                    match data[0] {
                                        16 => { // mouse input

                                        }
                                        _ => {
                                            println!("unknown packet id: {}", data[0]);
                                        }
                                    }
                                },
                                _ => {
                                    println!("received unknown message: {:?}", message);
                                    client.should_close = true;
                                }
                            }
                        }
                        Err(error) => {
                            match error {
                                websocket::WebSocketError::NoDataAvailable => {
                                break 'outer;
                                },
                                _ => {
                                    println!("read websocket message error: {}", error);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let mut server = algae::Server::new();
    server.start();
}
