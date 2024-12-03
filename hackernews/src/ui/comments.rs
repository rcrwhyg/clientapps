#![allow(non_snake_case)]

use crate::{ui::CommentsState, Comment};
use dioxus::prelude::*;

#[component]
pub fn Comments() -> Element {
    let comments_state = use_context::<Signal<CommentsState>>();

    match comments_state() {
        CommentsState::Unset => rsx! {
            div {}
        },
        CommentsState::Loading => rsx! {
            div { class: "mt-6",
                p { "Loading comments..." }
            }
        },
        CommentsState::Loaded(data) => rsx! {
            ul {
                for comment in data.comments {
                    StoryComment { comment }
                }
            }
        },
    }
}

#[component]
pub fn StoryComment(comment: Comment) -> Element {
    rsx! {
        li {
            article { class: "p-4 text-gray-500 leading-7 tracking-wider border-b border-gray-200",
                span { "{comment.by} {comment.time} | next [-]" }
                div {
                    dangerous_inner_html: comment.text
                }
            }
        }
    }
}
