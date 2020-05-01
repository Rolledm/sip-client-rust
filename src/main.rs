use orbtk::prelude::*;
use std::net::{TcpStream};
use std::io::{Read, Write};
use std::str::from_utf8;

#[derive(Debug, Copy, Clone)]
enum Action {
    Login,
}

enum AppState {
    Logging,
    Main,
}

#[derive(AsAny)]
pub struct MainViewState {
    app_state: AppState,
    action: Option<Action>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState { app_state: AppState::Logging, action: None }
    }
}

impl MainViewState {
    fn action(&mut self, action: impl Into<Option<Action>>) {
        self.action = action.into();
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if let Some(action) = self.action {
            match action {
                Action::Login => {
                    println!("Login");
                    println!("ext {}", ctx.widget().get_mut::<String16>("ext"));
                    println!("pass {}", ctx.widget().get_mut::<String16>("pass"));
                    ctx.widget().set("status_string", String16::from("Logging"));

                    match TcpStream::connect("localhost:7878") {
                        Ok(mut stream) => {
                            println!("Connected!");
                            let msg = b"Hello";
                            stream.write(msg).unwrap();
                            let mut data = [0; 10];
                            match stream.read(&mut data) {
                                Ok(_) => {
                                    let text = from_utf8(&data).unwrap();
                                    println!("Output from server: {}", text);
                                    self.app_state = AppState::Main;
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

                }
            }

            self.action = None;
        }
    }
}

widget!(MainView<MainViewState> {
    ext: String16,
    pass: String16,
    status_string: String16
});

impl Template for MainView {
    fn template(self, id: Entity, ctx: &mut BuildContext) -> Self {
        self.name("MainView").child(
            Stack::create()
                .orientation("vertical")
                // By injecting the id of the parent the text property
                // is shared between the MainView and the TextBox. This
                // means both references the same String16 object.
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
                    .text(("status_string", id))
                    .horizontal_alignment("center")
                    .class("h1")
                    .build(ctx),)
                .horizontal_alignment("center")
                .vertical_alignment("center")
                .build(ctx),
        )
    }
}

fn main() {
    Application::new()
        .window(|ctx| {
            Window::create()
                .title("SIP Client")
                .position((100.0, 100.0))
                .size(300.0, 300.0)
                .child(MainView::create().margin(4.0).build(ctx))
                .build(ctx)
        })
        .run();
}

/*fn state<'a>(id: Entity, states: &'a mut StatesContext) -> &'a mut MainViewState {
    states.get_mut(id);
}*/