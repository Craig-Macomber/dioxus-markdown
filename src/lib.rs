use rust_web_markdown::{render_markdown, CowStr};

use std::collections::BTreeMap;

pub type MdComponentProps = rust_web_markdown::MdComponentProps<Element>;

use core::ops::Range;

pub use rust_web_markdown::{
    ComponentCreationError, Context, ElementAttributes, HtmlElement, LinkDescription, Options,
};

use dioxus::prelude::*;

use std::rc::Rc;

pub type HtmlCallback<T> = Rc<dyn Fn(T) -> Element>;

#[cfg(feature = "debug")]
pub mod debug {
    #[derive(Clone)]
    pub struct EventInfo(pub Vec<String>);
}

#[derive(Clone, Props)]
pub struct MdProps {
    src: String,

    /// The callback called when a component is clicked.
    /// If you want to control what happens when a link is clicked,
    /// use [`render_links`][render_links]
    on_click: Option<EventHandler<MarkdownMouseEvent>>,

    ///
    render_links: Option<HtmlCallback<LinkDescription<Element>>>,

    /// the name of the theme used for syntax highlighting.
    /// Only the default themes of [syntect::Theme] are supported
    theme: Option<String>,

    /// wether to enable wikilinks support.
    /// Wikilinks look like [[shortcut link]] or [[url|name]]
    #[props(default = false)]
    wikilinks: bool,

    /// wether to convert soft breaks to hard breaks.
    #[props(default = false)]
    hard_line_breaks: bool,

    /// pulldown_cmark options.
    /// See [`Options`][pulldown_cmark_wikilink::Options] for reference.
    parse_options: Option<Options>,

    #[props(default)]
    components: CustomComponents,

    frontmatter: Option<Signal<String>>,
}

impl PartialEq for MdProps {
    fn eq(&self, other: &Self) -> bool {
        self.src == other.src
            && self.on_click == other.on_click
            && match (&self.render_links, &other.render_links) {
                (Some(a), Some(b)) =>  Rc::ptr_eq(a , b),
                (None, None) => true,
                _ => false,
            }
            && self.theme == other.theme
            && self.wikilinks == other.wikilinks
            && self.hard_line_breaks == other.hard_line_breaks
            && self.parse_options == other.parse_options
            && self.components == other.components
            && self.frontmatter == other.frontmatter
    }
}

#[derive(Clone, Debug)]
pub struct MarkdownMouseEvent {
    /// the original mouse event triggered when a text element was clicked on
    pub mouse_event: MouseEvent,

    /// the corresponding range in the markdown source, as a slice of [`u8`][u8]
    pub position: Range<usize>,
    // TODO: add a clonable tag for the type of the element
    // pub tag: pulldown_cmark::Tag,
}

#[derive(Clone, Copy)]
pub struct MdContext(pub Signal<MdProps>);

/// component store.
/// It is called when therer is a `<CustomComponent>` inside the markdown source.
/// It is basically a hashmap but more efficient for a small number of items
#[derive(Clone)]
pub struct CustomComponents(
    Rc<BTreeMap<
        &'static str,
        Box<dyn Fn( MdComponentProps) -> Result<Element, ComponentCreationError>>,
    >>,
);



impl PartialEq for CustomComponents {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.0 , &other.0)
    }
}

impl Default for CustomComponents {
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<'a> CustomComponents {
    /// register components.
    /// The function `component` takes a context and props of type `MdComponentProps`
    /// and returns html
    pub fn new<T: Iterator<Item=(&'static str, Box<dyn Fn( MdComponentProps) -> Result<Element, ComponentCreationError>>)>>(content: T) -> Self {
        Self(Rc::new(BTreeMap::from_iter(content)))
    }
}

impl<'a> Context<'a, 'a> for MdContext {
    type View = Element;

    type Handler<T: 'a> = EventHandler<T>;

    type MouseEvent = MouseEvent;

    #[cfg(feature = "debug")]
    fn send_debug_info(self, info: Vec<String>) {
        let mut debug = use_context_provider::<Signal<debug::EventInfo>>(|| Signal::new(debug::EventInfo(vec![])));
        // to avoid re-rendering the parent component
        // if not needed
        if *debug.read().0 != info {
            debug.write().0 = info
        }
    }

    fn el_with_attributes(
        self,
        e: HtmlElement,
        inside: Self::View,
        attributes: ElementAttributes<EventHandler<MouseEvent>>,
    ) -> Self::View {
        let class = attributes.classes.join(" ");
        let style = attributes.style.unwrap_or_default();
        let onclick = attributes.on_click.unwrap_or_default();
        let onclick = move |e| onclick.call(e);

        let vnode = match e {
            HtmlElement::Div => {
                rsx! {div {onclick:onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Span => {
                rsx! {span {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Paragraph => {
                rsx! {p {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::BlockQuote => {
                rsx! {blockquote {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Ul => {
                rsx! {ul {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Ol(x) => {
                rsx! {ol {onclick: onclick, style: "{style}", class: "{class}", start: x as i64, {inside} } }
            }
            HtmlElement::Li => {
                rsx! {li {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(1) => {
                rsx! {h1 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(2) => {
                rsx! {h2 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(3) => {
                rsx! {h3 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(4) => {
                rsx! {h4 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(5) => {
                rsx! {h5 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(6) => {
                rsx! {h6 {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Heading(_) => panic!(),
            HtmlElement::Table => {
                rsx! {table {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Thead => {
                rsx! {thead {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Trow => {
                rsx! {tr {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Tcell => {
                rsx! {td {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Italics => {
                rsx! {i {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Bold => {
                rsx! {b {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::StrikeThrough => {
                rsx! {s {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Pre => {
                rsx! {p {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
            HtmlElement::Code => {
                rsx! {code {onclick: onclick, style: "{style}", class: "{class}", {inside} } }
            }
        };

        vnode
    }

    fn el_span_with_inner_html(
        self,
        inner_html: String,
        attributes: ElementAttributes<EventHandler<MouseEvent>>,
    ) -> Self::View {
        let class = attributes.classes.join(" ");
        let style = attributes.style.unwrap_or_default();
        let onclick = move |e| {
            if let Some(f) = &attributes.on_click {
                f.call(e)
            }
        };
        rsx! {
            span {
                dangerous_inner_html: "{inner_html}",
                style: "{style}",
                class: "{class}",
                onclick: onclick
            }
        }
    }

    fn el_hr(self, attributes: ElementAttributes<EventHandler<MouseEvent>>) -> Self::View {
        let class = attributes.classes.join(" ");
        let style = attributes.style.unwrap_or_default();
        let onclick = move |e| {
            if let Some(f) = &attributes.on_click {
                f.call(e)
            }
        };
        rsx!(hr {
            onclick: onclick,
            style: "{style}",
            class: "{class}"
        })
    }

    fn el_br(self) -> Self::View {
        rsx!(br {})
    }

    fn el_fragment(self, children: Vec<Self::View>) -> Self::View {
        rsx! {{children.into_iter()}}
    }

    fn el_a(self, children: Self::View, href: String) -> Self::View {
        rsx! {a {href: "{href}", {children}}}
    }

    fn el_img(self, src: String, alt: String) -> Self::View {
        rsx!(img {
            src: "{src}",
            alt: "{alt}"
        })
    }

    fn el_text(self, text: CowStr<'a>) -> Self::View {
        rsx! {{text.as_ref()}}
    }

    fn mount_dynamic_link(self, rel: &str, href: &str, integrity: &str, crossorigin: &str) {
        // let create_eval = use_eval(self.0);

        // let eval = create_eval(
        //     r#"
        //     // https://stackoverflow.com/a/18510577
        //     let rel = await dioxus.recv();
        //     let href = await dioxus.recv();
        //     let integrity = await dioxus.recv();
        //     let crossorigin = await dioxus.recv();
        //     var newstyle = document.createElement("link"); // Create a new link Tag

        //     newstyle.setAttribute("rel", rel);
        //     newstyle.setAttribute("type", "text/css");
        //     newstyle.setAttribute("href", href);
        //     newstyle.setAttribute("crossorigin", crossorigin);
        //     newstyle.setAttribute("integrity", integrity);
        //     document.getElementsByTagName("head")[0].appendChild(newstyle);
        //     "#,
        // )
        // .unwrap();

        // // You can send messages to JavaScript with the send method
        // eval.send(rel.into()).unwrap();
        // eval.send(href.into()).unwrap();
        // eval.send(integrity.into()).unwrap();
        // eval.send(crossorigin.into()).unwrap();
    }

    fn el_input_checkbox(
        self,
        checked: bool,
        attributes: ElementAttributes<EventHandler<MouseEvent>>,
    ) -> Self::View {
        let class = attributes.classes.join(" ");
        let style = attributes.style.unwrap_or_default();
        let onclick = move |e| {
            if let Some(f) = &attributes.on_click {
                f.call(e)
            }
        };
        rsx!(input {
            r#type: "checkbox",
            checked: checked,
            style: "{style}",
            class: "{class}",
            onclick: onclick
        })
    }

    fn props(self) -> rust_web_markdown::MarkdownProps<'a> {
        let props = self.0.read().clone();

        rust_web_markdown::MarkdownProps {
            hard_line_breaks: props.hard_line_breaks,
            wikilinks: props.wikilinks,
            parse_options: props.parse_options.as_ref(),
            theme: props.theme.as_deref(),
        }
    }

    fn call_handler<T: 'a>(callback: &Self::Handler<T>, input: T) {
        callback.call(input)
    }

    fn make_md_handler(
        self,
        position: std::ops::Range<usize>,
        stop_propagation: bool,
    ) -> Self::Handler<MouseEvent> {
        let on_click = self.0.read().on_click.as_ref();

        self.0.event_handler(move |e: MouseEvent| {
            if stop_propagation {
                e.stop_propagation()
            }

            let report = MarkdownMouseEvent {
                position: position.clone(),
                mouse_event: e,
            };

            on_click.map(|x| x.call(report));
        })
    }

    fn set_frontmatter(self, frontmatter: String) {
        self.0
            .read()
            .frontmatter
            .as_ref()
            .map(|x| x.set(frontmatter));
    }

    fn has_custom_links(self) -> bool {
        self.0.read().render_links.is_some()
    }

    fn render_links(self, link: LinkDescription<Self::View>) -> Result<Self::View, String> {
        // TODO: remove the unwrap call
        Ok(self.0.read().render_links.as_ref().unwrap()(
            link,
        ))
    }

    fn has_custom_component(self, name: &str) -> bool {
        self.0.read().components.0.get(name).is_some()
    }

    fn render_custom_component(
        self,
        name: &str,
        input: rust_web_markdown::MdComponentProps<Self::View>,
    ) -> Result<Self::View, ComponentCreationError> {
        let props = self.0.read();
        let f = props.components.0.get(name).unwrap();
        f(input)
    }
}

#[component]
pub fn Markdown(cx: Signal<MdProps>) -> Element {
    let context = MdContext(cx);
    let props = cx.read();
    render_markdown(context, &props.src)
}
