extern crate ocbot;
extern crate irc;
extern crate eventsource;
extern crate reqwest;

use std::default::Default;
use irc::client::prelude::*;
use eventsource::reqwest::Client;
use reqwest::Url;
use ocbot::game_stream::{parse_event, GameEvent};

fn main() {
    for event in game_events("https://clonkspot.org/league/game_events.php") {
        match event {
            GameEvent::Init(_) => println!("init"),
            GameEvent::Create(game) => println!("create {}", game.title),
            _ => ()
        }
    }
    return;
    let config = Config {
        nickname: Some(format!("ocgames")),
        server: Some(format!("irc.euirc.net")),
        channels: Some(vec![format!("#clonkspot")]),
        port: Some(6667),
        .. Default::default()
    };
    let server = IrcServer::from_config(config).unwrap();
    server.identify().unwrap();
    for message in server.iter() {
        let message = message.unwrap(); // We'll just panic if there's an error.
        print!("{}", message);
        match message.command {
            Command::PRIVMSG(ref target, ref msg) => if msg.contains("pickles") {
                server.send_privmsg(target, "Hi!").unwrap();
            },
            _ => (),
        }
    }
}

fn game_events(url: &str) -> Box<Iterator<Item=GameEvent>> {
    let client = Client::new(Url::parse(url).unwrap()).unwrap();
    let r = client
        .flat_map(|e| parse_event(&e.unwrap()));
    Box::new(r)
}
