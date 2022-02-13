use dioxus::prelude::*;


fn main() {
    dioxus::web::launch(app);
}

fn app(cx: Scope) -> Element {
    cx.render(rsx! (
        style { [include_str!("../src/style.css")] }
        header {}
        section {
            div {class: "one"}
            div {class: "two"}
            div {class: "three"}
        }
        footer {}
    ))
}
