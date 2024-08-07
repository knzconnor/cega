use base64::{engine::general_purpose::STANDARD, Engine};

use web_sys::HtmlInputElement;
use yew::{html, Callback, Component, Context, Event, Html, Properties, SubmitEvent, TargetCast};

use crate::color::palette::palette_from_abbr;
use crate::parser::ParserType;
use crate::{file_data, png};

use crate::wasm::FileUpload;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub file: FileUpload,
}

pub struct ImageComponent {
    width: usize,
}

pub enum Msg {
    Width(usize),
}

impl ImageComponent {
    pub fn src(&self, file: &FileUpload) -> String {
        let data = if file.mime_type.contains("image") {
            file.data.clone()
        } else {
            let file_data = file_data::Raw::new(&file.data);
            let parser = ParserType::CGA;
            let image = file_data.parse(parser, self.width);
            let palette = palette_from_abbr("cga0");
            let mut bytes: Vec<u8> = Vec::new();

            let _ = png::write_to(&mut bytes, image.data(), palette.clone());
            bytes
        };
        format!("data:application/png;base64,{}", STANDARD.encode(data))
    }

    pub fn previews(&self, ctx: &Context<Self>, file: &FileUpload) -> Html {
        if file.mime_type.contains("image") {
            "".into()
        } else {
            let file_data = file_data::Raw::new(&file.data);
            file_data
                .previews()
                .iter()
                .map(|p| {
                    let palette = palette_from_abbr("cga0");
                    let mut bytes: Vec<u8> = Vec::new();

                    let _ = png::write_to(&mut bytes, p.data(), palette.clone());
                    let src = format!("data:application/png;base64,{}", STANDARD.encode(bytes));
                    let width = p.width().clone();
                    html! {
                        <div class="preview-tile" onclick={ctx.link().callback( move |_|{
                            Msg::Width(width)
                        })}>
                            <h3>{&width.to_string()}</h3>
                            <img src={ src } />
                        </div>
                    }
                })
                .collect()
        }
    }
}

impl Component for ImageComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self { width: 320 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Width(w) => {
                self.width = w;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let noop = Callback::from(|e: SubmitEvent| {
            e.prevent_default();
        });
        let onchange = ctx.link().callback(|e: Event| {
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::Width(input.value().parse().expect("fail to parse width"))
        });
        let file = &ctx.props().file;

        html! {
            <>
                <div class="preview-tile">
                    <div class=".preview-media">
                        <p class="preview-name">{ file.name.to_string() }</p>
                        <img src={ self.src(file) } />
                    </div>
                    <form onsubmit={noop}>
                            <label for="width">{"Width"}</label>
                            <input name="width" type="number" value={self.width.to_string()} {onchange} />
                    </form>
                </div>
                <div class="preview-row">
                    {self.previews(ctx, file)}
                </div>
            </>
        }
    }
}
