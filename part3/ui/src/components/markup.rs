use yew::prelude::*;

use pulldown_cmark::{html, Options, Parser};
use web_sys::Node;
use yew::virtual_dom::vnode::VNode;

use reqwasm::http::Request;

use crate::utils::fetchstates::{FetchError, FetchState, FetchStateMsg};

#[derive(Clone, Debug, Eq, PartialEq, Properties)]
pub struct Props {
    pub id: String,
}

pub struct Markup {
    markup: FetchState<common::Markup>,
}

impl Component for Markup {
    type Message = FetchStateMsg<common::Markup>;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            markup: FetchState::NotFetching,
        }
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        ctx.link().send_message(FetchStateMsg::GetData);
        true
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        let uri: String = format!("/api/markup/{}", _ctx.props().id);

        match _msg {
            FetchStateMsg::SetDataFetchState(state) => {
                self.markup = state;
                true
            }
            FetchStateMsg::GetData => {
                _ctx.link().send_future(async move {
                    match Request::get(uri.as_str()).send().await {
                        Ok(makrup) => match makrup.json().await {
                            Ok(makrup) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Success(makrup))
                            }
                            Err(err) => {
                                FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                                    err: err.to_string(),
                                }))
                            }
                        },
                        Err(err) => {
                            FetchStateMsg::SetDataFetchState(FetchState::Failed(FetchError {
                                err: err.to_string(),
                            }))
                        }
                    }
                });
                _ctx.link()
                    .send_message(FetchStateMsg::SetDataFetchState(FetchState::Fetching));
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        if matches!(&self.markup, &FetchState::NotFetching) {
            _ctx.link().send_message(FetchStateMsg::GetData);
        }
        let div = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .create_element("div")
            .unwrap();
        div.set_inner_html(&self.render_markdown());
        let node = Node::from(div);
        let vnode = VNode::VRef(node);
        vnode
    }
}

impl Markup {
    fn render_markdown(&self) -> String {
        let Self { markup } = self;
        let mut options = Options::empty();
        options.insert(Options::ENABLE_STRIKETHROUGH);

        match markup {
            FetchState::Success(markup) => {
                let markdown = markup.markup.clone();
                let parser = Parser::new_ext(&markdown[..], options);

                let mut html_output = String::new();
                html::push_html(&mut html_output, parser);
                html_output
            }
            _ => "Failed to Load ...".to_string(),
        }
    }
}
