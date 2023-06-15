#![windows_subsystem = "windows"]
use std::io;
use std::io::ErrorKind;
use std::ops::Add;
use gtk::glib;
use gtk::prelude::*;

use glib::Type;
use gtk::gio::SimpleAction;
use gtk::{Application, Entry, Label, Orientation};
use gtk::MessageType::Error;
use gtk::PadActionType::Button;
use ureq;
use tl;
use tl::Attributes;
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use serde_json::{json, Value};

fn main() -> glib::ExitCode {
    let application = Application::new(
        Some("com.comrade.brickhilllauncher"),
        Default::default(),
    );
    application.connect_activate(build_ui);

    // When activated, shuts down the application
    let quit = SimpleAction::new("quit", None);
    quit.connect_activate(
        glib::clone!(@weak application => move |_action, _parameter| {
            application.quit();
        }),
    );
    application.set_accels_for_action("app.quit", &["<Primary>Q"]);
    application.add_action(&quit);

    // Run the application
    application.run()
}

fn decode_b64(b64: String) -> String
{
    general_purpose::STANDARD
        .decode(b64.as_str())
        .unwrap()
        .iter()
        .map(|i| *i as char)
        .map(|i| i.to_string())
        .collect()
}

fn get_attribute_key(attr: &Attributes, key: &str) -> String
{
    attr.get(key)
        .unwrap()
        .unwrap()
        .as_utf8_str()
        .to_string()
}

fn launch_game(button: &gtk::Button, in_field: &Entry)
{
    let json_token = std::path::Path::new("config.json").exists();
    let token: String;
    if !json_token
    {
        let new_config = json!({"token": "INSERT_YOURS_HERE"}).to_string();
        std::fs::write("config.json", new_config).expect("Unable to create new config for some reason.");
        button.set_label("Config file not found! Creating one...");
        return;
    }
    else
    {
        let json_config_file = std::fs::read_to_string("config.json").expect("Unable to read config file for some reason.");
        let json_config: Value = serde_json::from_str(json_config_file.as_str()).unwrap();
        token = json_config["token"].to_string();
        if token.contains("INSERT_YOURS_HERE")
        {
            button.set_label("You need to add your token to the config file.");
            return;
        }
    }

    let in_url = in_field.text().to_string();
    let mut game_token: String = String::new();
    let mut game_ip: String = String::new();
    let mut game_port: String = String::new();
    if (in_url.eq("localhost"))
    {
        game_token = String::from("local");
        game_ip = String::from("local");
        game_port = String::from("42480");
    }
    else
    {
        if !(in_url.starts_with("http")) || !(in_url.contains("brick-hill"))
        {
            button.set_label("Invalid Brick Hill URL!");
            return;
        }
        let game_page_req = ureq::get(in_url.as_str()).call().unwrap().into_string().unwrap();
        let game_page_dom = tl::parse(game_page_req.as_str(), tl::ParserOptions::default()).unwrap();
        let game_page_parser = game_page_dom.parser();
        let game_page_details = game_page_dom.get_element_by_id("setpage-v")
            .unwrap()
            .get(game_page_parser)
            .unwrap();
        let game_page_attributes = game_page_details.as_tag().unwrap().attributes();
        let game_id = get_attribute_key(game_page_attributes, ":set-id");
        let game_ip = decode_b64(get_attribute_key(game_page_attributes, "set-ip").chars().rev().collect::<String>());
        let game_port = get_attribute_key(game_page_attributes, "set-port");

        let token_request_url = format!("https://api.brick-hill.com/v1/auth/generateToken?set={}", game_id);
        let token_client = ureq::get(token_request_url.as_str())
            .set("Cookie", format!("brick_hill_session={}", token).as_str())
            .call()
            .unwrap()
            .into_string();
        let token_request = token_client.unwrap();
        let token_request_json: Value = serde_json::from_str(token_request.as_str()).unwrap();
        let game_token = token_request_json.get("token").unwrap().as_str().unwrap().to_string();
    }

    let launcher_arg = format!("brickhill.legacy://client/{}/{}/{}", game_token, game_ip, game_port);
    println!("{}", launcher_arg);

    if cfg!(target_os = "windows")
    {
        let launcher_cmd = std::process::Command::new("C:/Program Files (x86)/Brick Hill/legacy_autoupdater.exe")
            .arg(launcher_arg)
            .spawn()
            .unwrap();
    }
    button.set_label("Have fun!")
}

fn build_ui(application: &Application) {
    // create the main window
    let window = gtk::ApplicationWindow::builder()
        .application(application)
        .title("Brick Hill Crossover Launcher")
        .default_width(600)
        .default_height(160)
        .build();

    // Create a title label
    let win_title = Label::new(None);
    win_title.set_markup("<big>Enter Brick Hill game url.</big>");

    let input_field = Entry::new();
    let go_button = gtk::Button::builder()
        .label("Launch Game")
        .halign(gtk::Align::Center)
        .valign(gtk::Align::Center)
        .build();

    let row = gtk::Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();
    row.append(&win_title);
    input_field.set_margin_top(10);
    row.append(&input_field);
    row.append(&go_button);

    go_button.connect_clicked(move |but| {
        launch_game(&but, &input_field);
    });

    window.set_child(Some(&row));

    // show everything
    window.present();
}