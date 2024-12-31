#![allow(non_snake_case)]
use dioxus::prelude::*;

use dioxus_markdown::*;

static MARKDOWN_SOURCE: &str = r#"
## Here is a counter:
<Counter initial="5"/>

<Counter initial="a"/>

<Counter/>

## Here is a Box:
<box>

**I am in a blue box !**

</box>
"#;

#[component]
fn Counter(initial: i32) -> Element {
    let mut count = use_state(cx, || *initial);

   rsx!{
        div{
            button {
                onclick: move |_| count-=1,
                "-"
            },
            "{count}",
            button {
                onclick: move |_| count+=1,
                "+"
            }
        }
    }
}

#[component]
fn ColorBox( children: Element) -> Element {
    rsx!{
        div{
            style: "border: 2px solid blue",
            {children}
        }
    }
}

// create a component that renders a div with the text "Hello, world!"
fn App(cx: Scope) -> Element {

    let custom = vec![(
        "Counter",
        |props: MdComponentProps| Ok(rsx!{
            Counter {initial: props.get_parsed_optional("initial")?.unwrap_or(0)}
        })
    ),(
        "box",
         |props: MdComponentProps| Ok(rsx!{
             ColorBox {children: props.children}
         })
     )];

    let mut components = CustomComponents::new(custom);


    cx.render(rsx! {
        h1 {"Source"}
        Markdown {
            src: "```md\n{MARKDOWN_SOURCE}\n``"
        }

        h1 {"Result"}
        Markdown {
            src: MARKDOWN_SOURCE,
            components: components
        }
    })
}

fn main() {
    // launch the web app
    dioxus_web::launch(App);
}
