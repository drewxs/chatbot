use leptos::html::{Div, Input};
use leptos::*;
use uuid::Uuid;

use crate::{
    api::converse,
    models::conversation::{Conversation, Message},
};

#[component]
pub fn ChatPage(cx: Scope) -> impl IntoView {
    let (conversation, set_conversation) = create_signal(cx, Conversation::new());
    let chat_area_ref = create_node_ref::<Div>(cx);
    let input_ref = create_node_ref::<Input>(cx);

    let send = create_action(cx, move |new_message: &String| {
        let user_message = Message {
            id: Uuid::new_v4().to_string(),
            user: true,
            text: new_message.clone(),
        };
        set_conversation.update(move |c| c.messages.push(user_message));

        converse(cx, conversation.get())
    });

    create_effect(cx, move |_| {
        if let Some(_) = send.input().get() {
            let model_message = Message {
                id: Uuid::new_v4().to_string(),
                text: String::from("..."),
                user: false,
            };

            set_conversation.update(move |c| c.messages.push(model_message));
        }
    });

    create_effect(cx, move |_| {
        if let Some(Ok(response)) = send.value().get() {
            set_conversation.update(move |c| c.messages.last_mut().unwrap().text = response);
        }
    });

    create_effect(cx, move |_| {
        conversation.get();
        if let Some(div) = chat_area_ref.get() {
            div.set_scroll_top(div.scroll_height());
        }
    });

    view! { cx,
        <section>
            <div node_ref=chat_area_ref>
                {move || conversation.get().messages.iter().map(move |message| {
                    view! { cx,
                        <div class={if message.user { "user" } else { "bot" }}>
                            {message.text.clone()}
                        </div>
                    }
                }).collect::<Vec<_>>()}
            </div>
            <div>
                <form on:submit=move |e| {
                    e.prevent_default();
                    let input = input_ref.get().expect("input to exist");
                    send.dispatch(input.value());
                    input.set_value("");
                }>
                    <input type="text" node_ref=input_ref/>
                    <button type="submit">Send</button>
                </form>
            </div>
        </section>
    }
}
