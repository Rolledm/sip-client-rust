use orbtk::prelude::*;

#[derive(Debug, Copy, Clone)]
enum Action {
    Login,
}

#[derive(AsAny)]
pub struct MainViewState {
    action: Option<Action>,
}

impl Default for MainViewState {
    fn default() -> Self {
        MainViewState { action: None }
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

                }
            }

            self.action = None;
        }
    }
}

widget!(MainView<MainViewState> {
    ext: String16,
    pass: String16
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
                    // mouse click event handler
                    .on_click(move |states, entity| {
                        // Calls clear of the state of MainView
                        states.get_mut::<MainViewState>(id).action(Action::Login);
                        //println!("print");
                        //state(id, states).action(Action::Login);
                        true
                    })
                    .text("Login")
                    .icon(material_font_icons::KEYBOARD_ARROW_RIGHT_FONT_ICON)
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