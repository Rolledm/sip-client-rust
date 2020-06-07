use orbtk::prelude::*;
use orbtk::theme::DEFAULT_THEME_CSS;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;
use std::fs;
use xmltree::Element;

static MY_CSS: &'static str = include_str!("./main.css");


fn get_theme() -> ThemeValue {
    ThemeValue::create_from_css(DEFAULT_THEME_CSS)
        .extension_css(MY_CSS)
        .build()
}


#[derive(Debug, Copy, Clone)]
enum Action {
    Login,
    Call,
    ChangePresence,
    ChangeDND,
    Cancel,
    Update,
}

enum AppState {
    Logging,
    Main,
}

#[derive(AsAny)]
pub struct MainViewState {
    app_state: AppState,
    action: Option<Action>,
    presence: String,
    dnd: bool,
    stream: Option<TcpStream>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState { app_state: AppState::Logging, 
                        action: None, 
                        presence: String::from("available"), 
                        dnd: false, 
                        stream: None }
    }
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }

    fn login(&self, ext: &String, pass: &String) -> Result<(), &'static str> {
        println!("your ext = {}\nyour pass = {}\n", ext, pass);
        let mut message = sip_rld::Message::new(sip_rld::MessageType::Request(sip_rld::RequestMethod::Register), String::from("my.dom.ru"));
    
        message.to(None, ext.to_string())
                .via("TCP".to_string(), "localhost".to_string(), "5060".to_string())
                .max_forwards("70".to_string())
                .cseq("1".to_string())
                .call_id("HARDCODED".to_string())
                .request_uri(ext.to_string());
    
    
        match TcpStream::connect("localhost:5060") {
            Ok(mut stream) => {
                println!("Connected!");
                let msg = message.build_message();
                stream.write(msg.as_bytes()).unwrap();
                let mut data = [0; 1024];
                match stream.read(&mut data) {
                    Ok(_) => {
                        let text = String::from_utf8_lossy(&data);
                        let message = sip_rld::Message::parse(&text);
                        println!("{:?}", message);
                        println!("{}", text);
                        match message.mtype {
                            sip_rld::MessageType::Request(_) => return Err("Invalid message received"),
                            sip_rld::MessageType::Response(resp) => {
                                if resp == "200 OK" {
                                    println!("Successfully registered!");
                                    return Ok(());
                                } else {
                                    println!("Invalid response: {}", resp);
                                    return Err("Invalid response");
                                }
                            }
                        }
                        //self.app_state = AppState::Main;
                    },
                    Err(e) => {
                        println!("Err: {}", e);
                    }
                }
            },
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
        Err("Register failed")
    }
}

widget!(MainView<MainViewState> {
    ext: String16,
    pass: String16,
    login_status_string: String16,
    logged_ext: String16,
    call_ext: String16,
    presence_text_input: String16,
    status_line: String16,
    dnd: String16,
    presence: String16
    //main_status_string: String16,
    //feature_status_string: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Grid::create()
                .columns(
                    Columns::create()
                        .column(150.0)
                        .column("*")
                        .build()
                )
                .rows(Rows::create().row("*").row("*").build())
                .child(
                    Grid::create()
                    .element("login-screen")
                    .attach(Grid::column(0))
                    .child(
                            Stack::create()
                                .orientation("vertical")
                                .child(TextBox::create()
                                    .height(8.0)
                                    .water_mark("Ext.")
                                    .margin(2.0)
                                    .text(("ext", id))
                                    .build(ctx))
                                .child(TextBox::create()
                                    .height(8.0)
                                    .margin(2.0)
                                    .text(("pass", id))
                                    .water_mark("Pass.")
                                    .build(ctx))
                                .child(Button::create()
                                    .margin(2.0)
                                    .on_click(move |states, _| {
                                        states.get_mut::<MainViewState>(id).action(Action::Login);
                                        true
                                    })
                                    .text("Login")
                                    .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
                                    .build(ctx),)
                                .child(TextBlock::create()
                                    .height(8.0)
                                    .margin(5.0)
                                    .text(("login_status_string", id))
                                    .horizontal_alignment("center")
                                    .class("h1")
                                    .build(ctx),)
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                    )
                    .build(ctx),
                )
                .child(
                    Grid::create()
                    .element("main-screen")
                    .attach(Grid::column(1))
                    .child(
                            Stack::create()
                                .orientation("vertical")
                                .child(TextBlock::create()
                                    .height(8.0)
                                    .margin(5.0)
                                    .text(("status_line", id))
                                    .horizontal_alignment("left")
                                    .class("h3")
                                    .build(ctx),)
                                .child(TextBlock::create() // ext handler
                                    .height(8.0)
                                    .margin(5.0)
                                    .text(("logged_ext", id))
                                    .horizontal_alignment("left")
                                    .class("h3")
                                    .build(ctx),)
                                .child(TextBox::create() // change .text
                                    .height(8.0)
                                    .margin(2.0)
                                    .text(("call_ext", id))
                                    .horizontal_alignment("left")
                                    .water_mark("dial...")
                                    .build(ctx))
                                .child(Stack::create()
                                    .orientation("horizontal")
                                    .horizontal_alignment("center")
                                    .child(Button::create()
                                        .margin(2.0)
                                        .on_click(move |states, _| {
                                            states.get_mut::<MainViewState>(id).action(Action::Call);
                                            true
                                        })
                                        .text("Call")
                                        .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
                                        .build(ctx),)
                                    .child(Button::create()
                                        .margin(2.0)
                                        .on_click(move |states, _| {
                                            states.get_mut::<MainViewState>(id).action(Action::Cancel);
                                            true
                                        })
                                        .text("Cancel")
                                        .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
                                        .build(ctx),)
                                    .build(ctx),)
                                .horizontal_alignment("center")
                                .vertical_alignment("center")
                                .build(ctx),
                    )
                    .build(ctx),
                )
                .child(
                    Grid::create()
                    .element("feature-screen")
                    .attach(Grid::column(0))
                    .attach(Grid::row(1))
                    .attach(Grid::column_span(2))
                    .child(
                        Stack::create()
                        .orientation("vertical")
                        .child(TextBlock::create() // ext handler
                            .height(8.0)
                            .margin(5.0)
                            .text(("presence", id))
                            .horizontal_alignment("left")
                            .class("h3")
                            .build(ctx),)
                        .child(Stack::create()
                            .orientation("horizontal")
                            .horizontal_alignment("left")
                            .child(TextBox::create() // change .text
                                .height(8.0)
                                .margin(2.0)
                                .text(("presence_text_input", id))
                                .horizontal_alignment("left")
                                .water_mark("Change presence...")
                                .build(ctx))
                            .child(Button::create()
                                .margin(2.0)
                                .on_click(move |states, _| {
                                    states.get_mut::<MainViewState>(id).action(Action::ChangePresence);
                                    true
                                })
                                .text("Change")
                                .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
                                .build(ctx),)
                            .build(ctx),)
                        .child(Stack::create()
                            .orientation("horizontal")
                            .horizontal_alignment("left")
                            .child(TextBlock::create() // ext handler
                                .height(8.0)
                                .margin(5.0)
                                .text(("dnd", id))
                                .horizontal_alignment("left")
                                .class("h3")
                                .build(ctx),)
                            .child(Button::create()
                                .margin(2.0)
                                .on_click(move |states, _| {
                                    states.get_mut::<MainViewState>(id).action(Action::ChangeDND);
                                    true
                                })
                                .text("Change")
                                .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
                                .build(ctx),)
                            .build(ctx),)
                        .child(TextBlock::create()
                            .height(8.0)
                            .margin(5.0)
                            .text("1176: available")
                            .horizontal_alignment("left")
                            .class("h3")
                            .build(ctx),)
                        .horizontal_alignment("left")
                        .vertical_alignment("center")
                        .build(ctx),
                    )
                    .build(ctx),
                )
                .build(ctx),
        )
    }
}

impl State for MainViewState {
    fn init(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        ctx.widget().set("logged_ext", String16::from(format!("Ext: ")));
        ctx.widget().set("status_line", String16::from("Status line..."));
        ctx.widget().set("dnd", String16::from("DND: unknown"));
        ctx.widget().set("presence", String16::from("Your presence: unknown"));
    }


    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action { //login_status_string
                Action::Login => {
                    match self.app_state {
                        AppState::Logging => {
                            ctx.widget().set("login_status_string", String16::from("Logging"));
                            let ext = ctx.widget().get_mut::<String16>("ext").as_string();
                            let pass = ctx.widget().get_mut::<String16>("pass").as_string();
                            let login = self.login(&ext, &pass);
                            match login {
                                Ok(_) => {
                                    ctx.widget().set("login_status_string", String16::from("Success!"));
                                    ctx.widget().set("logged_ext", String16::from(format!("Ext: {}", ext)));
                                    self.app_state = AppState::Main;
                                    
                                    // move to sep func
                                    ctx.widget().set("presence", String16::from(format!("Your presence: {}", self.presence.clone())));
                                    if self.dnd {
                                        ctx.widget().set("dnd", String16::from("DND: enabled"));
                                    } else {
                                        ctx.widget().set("dnd", String16::from("DND: disabled"));
                                    }
                                },
                                Err(e) => {
                                    println!("Error: {}", e);
                                    ctx.widget().set("login_status_string", String16::from("Error"));
                                }
                            }
                        },
                        _ => ctx.widget().set("status_line", String16::from("Already logged in"))
                    };
                },
                Action::Call => {
                    println!("Call {}", ctx.widget().get_mut::<String16>("call_ext").as_string())
                },
                Action::ChangePresence => {
                    let presence = ctx.widget().get_mut::<String16>("presence_text_input").as_string();
                    self.presence = presence;
                    ctx.widget().set("presence", String16::from(format!("Your presence: {}", self.presence.clone())));
                },
                Action::ChangeDND => {
                    self.dnd = !self.dnd;
                    if self.dnd {
                        ctx.widget().set("dnd", String16::from("DND: enabled"));
                    } else {
                        ctx.widget().set("dnd", String16::from("DND: disabled"));
                    }
                },
                Action::Cancel => {
                    println!("Cancel")
                },
                Action::Update => {
                    
                }
            }

            self.action = None;
        }
    }
}

fn main() {
    let text = load_file("./config/settings.xml");
    parse(&text);

    Application::new()
        .window(|ctx| {
            Window::create()
                .title("SIP Client")
                .position((100.0, 100.0))
                .size(500.0, 320.0)
                .theme(get_theme())
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}

fn parse(text: &str) {
    let domain = Element::parse(text.as_bytes()).unwrap();//.get_child("domain");
    println!("{}", domain.get_child("domain").unwrap().get_text().unwrap());
}

fn load_file(path: &str) -> String {
    let mut file = fs::File::open(path).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text
}