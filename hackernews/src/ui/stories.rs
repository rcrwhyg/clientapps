#![allow(non_snake_case)]

use super::CommentsState;
use crate::{
    api::{get_story_comments, get_top_stories},
    StoryData, StoryItem,
};
use dioxus::prelude::*;
use tracing::info;

#[component]
pub fn Stories() -> Element {
    let stories = use_resource(move || get_top_stories(20));
    match &*stories.read_unchecked() {
        Some(Ok(stories)) => rsx! {
            ul {
                for story in stories {
                    StoryItemComponent { story: story.clone() }
                }
            }
        },
        Some(Err(err)) => rsx! {
            div { class: "mt-6 text-red-500",
                p { "Failed to fetch stories" }
                p { "{err}" }
            }
        },
        None => rsx! {
            div { class: "mt-6",
                p { "Loading stories..." }
            }
        },
    }
}

#[component]
pub fn StoryItemComponent(story: StoryItem) -> Element {
    let mut comments_state = use_context::<Signal<CommentsState>>();
    // cache of the already loaded comments: Option<StoryData>
    // let full_story = use_signal(|| None);
    rsx! {
        li { class: "py-5 border-b px-3 transition hover:bg-indigo-100",
            a { href: "#", class: "flex justify-between items-center",
                h3 { class: "text-lg font-semibold", "{story.title}" }
                p { class: "text-md text-gray-400" }
            }
            div { class: "text-md italic text-gray-400",
                span {"{story.score} points by {story.by} {story.time} | "}
                a {
                    href: "#",
                    onclick: move |e| {
                        e.prevent_default();
                        let story = story.clone();
                        info!("clicked on story: {} with event: {:#?}", story.title, e);

                        // load_comments(comments_state, story);
                        async move {
                            *comments_state.write() = CommentsState::Loading;

                            if let Ok(story_data) = get_story_comments(story).await {
                                *comments_state.write() = CommentsState::Loaded(Box::new(story_data));
                            }
                        }
                    },
                    "{story.kids.len()} comments"
                }
            }
        }
    }
}

#[allow(unused)]
async fn load_comments(
    mut comments_state: Signal<CommentsState>,
    mut full_story: Signal<Option<StoryData>>,
    story: StoryItem,
) {
    // if the comments are already loaded, just change comments_state and return
    if let Some(story_data) = full_story.as_ref() {
        *comments_state.write() = CommentsState::Loaded(Box::new(story_data.clone()));
        return;
    }

    // if no, set comments_state to Loading and fetch the comments
    *comments_state.write() = CommentsState::Loading;

    if let Ok(story_data) = get_story_comments(story).await {
        *comments_state.write() = CommentsState::Loaded(Box::new(story_data.clone()));
        *full_story.write() = Some(story_data);
    }
}
