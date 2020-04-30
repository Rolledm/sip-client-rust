use orbtk::prelude::*;

#[derive(Default, AsAny)]
pub struct MainViewState {
    clear: bool,
}

impl MainViewState {
    // Sets an action the state
    fn clear(&mut self) {
        self.clear = true;
    }
}

impl State for MainViewState {
    fn update(&mut self, _: &mut Registry, ctx: &mut Context<'_>) {
        if self.clear {
            // Clears the text property of MainView and because
            // of the sharing also the text of the TextBox.
            ctx.widget().set("text", String16::from(""));
            self.clear = false;
        }
    }
}

widget!(MainView<MainViewState> {
    text: String16
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
                    .text(id)
                    .build(ctx))
                .child(TextBox::create()
                    .height(8.0)
                    .margin(2.0)
                    .water_mark("Pass.")
                    .build(ctx))
                .child(Button::create()
                    .margin(2.0)
                    // mouse click event handler
                    .on_click(move |states, _| {
                        // Calls clear of the state of MainView
                        states.get_mut::<MainViewState>(id);
                        println!("print");
                        true
                    })
                    .text("Login")
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