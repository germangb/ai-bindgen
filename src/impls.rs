use crate::{Attributes, Error, Transform, credentials::Credentials};
use openai_api_rust::{
    Auth, Message, OpenAI, Role,
    chat::{ChatApi, ChatBody},
};
use proc_macro2::TokenStream;
use quote::quote;
use std::path::PathBuf;
use syn::{ForeignItemFn, ItemForeignMod, spanned::Spanned};

impl Transform for ItemForeignMod {
    fn try_transform(self, attr: TokenStream) -> Result<TokenStream, Error> {
        let ItemForeignMod { items, .. } = self;

        if !attr.is_empty() {
            Err(Error::Syn(syn::Error::new(
                attr.span(),
                "Parameters are not supported in top-level extern block.",
            )))
        } else {
            // emit the inner items only
            Ok(quote!(#(#items)*))
        }
    }
}

impl Transform for ForeignItemFn {
    fn try_transform(self, attr: TokenStream) -> Result<TokenStream, Error> {
        // load the context
        let attr = Attributes::new(attr)?;
        let mut path = PathBuf::from(self.span().file());
        if let Some(context) = &attr.context {
            path.pop();
            path.push(context);
        }
        let context = std::fs::read_to_string(path)?;

        let ForeignItemFn {
            attrs, vis, sig, ..
        } = self;

        // generate contents
        let signature = quote!(#(#attrs)* #vis #sig).to_string();
        let tokens: TokenStream = syn::parse_str(&chat_completion(&attr, &signature, &context)?)?;

        // ignore warnings because we cant see the code anyway
        Ok(quote!(#(#attrs)* #[allow(warnings)] #vis # sig { #tokens }))
    }
}

fn chat_completion(attr: &Attributes, signature: &str, context: &str) -> Result<String, Error> {
    // init openai connection
    let credentials = Credentials::from_env()?;
    let openai = OpenAI::new(Auth::new(&credentials.api_key), &credentials.api_url);

    // make the chat request
    let prompt = attr
        .prompt
        .as_ref()
        .map(String::as_str)
        .unwrap_or_else(|| "No explanation given, figure it out from the function signature");
    let request = ChatBody {
        model: attr.model.clone().unwrap_or_else(|| credentials.api_model),
        max_tokens: attr.max_tokens,
        temperature: attr.temperature,
        top_p: attr.top_p,
        n: None,
        stream: None,
        stop: None,
        presence_penalty: attr.presence_penalty,
        frequency_penalty: attr.frequency_penalty,
        logit_bias: None,
        user: None,
        messages: vec![Message {
            role: Role::User,
            content: make_message(&signature, prompt, context),
        }],
    };

    openai
        .chat_completion_create(&request)?
        .choices
        .pop() // any choice is good!
        .and_then(|c| c.message)
        .map(|m| m.content)
        .ok_or(Error::Other("Unable to generate rust code".to_string()))
}

fn make_message(signature: &str, prompt: &str, context: &str) -> String {
    format!(
        include_str!("prompt.txt"),
        signature = signature,
        prompt = prompt,
        context = context
    )
}
