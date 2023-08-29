use cfg_if::cfg_if;
use leptos::*;

use crate::models::conversation::Conversation;

#[server(Converse "/api")]
pub async fn converse(cx: Scope, prompt: Conversation) -> Result<String, ServerFnError> {
    use actix_web::{dev::ConnectionInfo, web::Data};
    use leptos_actix::extract;
    use llm::{models::Llama, KnownModel};

    let model = extract(cx, |data: Data<Llama>, _connection: ConnectionInfo| async {
        data.into_inner()
    })
    .await
    .unwrap();

    let bot_name = "Ciel";
    let user_name = "User";
    let persona = "A chat between a human and his cat";
    let mut history = format!(
        "{bot_name}: Meow\n
        {user_name}: Hi {bot_name}!\n",
    );

    for message in prompt.messages.into_iter() {
        let msg = message.text;
        let name = if message.user { user_name } else { bot_name };
        let curr_line = format!("{user_name}: {msg}\n");

        history.push_str(&curr_line);
    }

    let mut rng = rand::thread_rng();
    let mut res = String::new();
    let mut buf = String::new();

    let mut session = (*model).start_session(Default::default());
    session
        .infer(
            model.as_ref(),
            &mut rng,
            &llm::InferenceRequest {
                prompt: format!("{persona}\n{history}\n{bot_name}:").as_str().into(),
                parameters: &llm::InferenceParameters::default(),
                play_back_previous_tokens: false,
                maximum_token_count: None,
            },
            &mut Default::default(),
            inference_callback(String::from(user_name), &mut buf, &mut res),
        )
        .unwrap_or_else(|e| panic!("{e}"));

    Ok(res)
}

cfg_if! {
    if #[cfg(feature = "ssr")] {
        use std::convert::Infallible;
        use llm::{InferenceRequest, InferenceFeedback};

        fn inference_callback<'a>(
            stop_sequence: String,
            buf: &'a mut String,
            out_str: &'a mut String,
        ) -> impl FnMut(InferenceRequest) -> Result<InferenceFeedback, Infallible> + 'a {
            use llm::{InferenceFeedback::{self, Continue, Halt}, InferenceResponse};

            move |res| match res {
                InferenceResponse::InferredToken(t) => {
                    let mut reverse_buf = buf.clone();
                    reverse_buf.push_str(t.as_str());
                    if stop_sequence.as_str().eq(reverse_buf.as_str()) {
                        buf.clear();
                        return Ok::<InferenceFeedback, Infallible>(Halt);
                    } else if stop_sequence.as_str().starts_with(reverse_buf.as_str()) {
                        buf.push_str(t.as_str());
                        return Ok(Continue);
                    }

                    let str = if buf.is_empty() { &t } else { &reverse_buf }
                    out_str.push_str(str);

                    Ok(Continue)
                }
                InferenceResponse::EotToken => Ok(Halt),
                _ => Ok(Continue),
            }
        }
    }
}
