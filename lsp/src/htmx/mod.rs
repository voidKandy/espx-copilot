use log::debug;
use lsp_types::TextDocumentPositionParams;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf, sync::OnceLock};

use crate::{
    agent::{block_prompt, AGENT},
    tree_sitter::Position,
};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EspxCompletion {
    pub name: String,
    pub desc: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EspxHover {
    pub name: String,
    pub desc: String,
}

impl From<&(&str, &str)> for EspxCompletion {
    fn from((name, desc): &(&str, &str)) -> Self {
        Self {
            name: name.to_string(),
            desc: desc.to_string(),
        }
    }
}

impl TryFrom<&(PathBuf, String)> for EspxCompletion {
    type Error = anyhow::Error;

    fn try_from((path, desc): &(PathBuf, String)) -> Result<Self, Self::Error> {
        match path.to_str() {
            None | Some("") => anyhow::bail!("Invalid path"),
            Some(name) => Ok(Self {
                name: name.to_string(),
                desc: desc.to_string(),
            }),
        }
    }
}

pub fn espx_completion(text_params: TextDocumentPositionParams) -> Option<Vec<EspxCompletion>> {
    let result = crate::tree_sitter::get_position_from_lsp_completion(text_params.clone())?;

    debug!("result: {:?} params: {:?}", result, text_params);

    match result {
        Position::AttributeName(name) => {
            if name.starts_with("hx-") {
                // let test = vec![("hx-boost", "YOUR MOTHER"), ("hx-get", "HIS MOTHER")];
                // Some(to_hx_completion(test))
                return HX_TAGS.get().cloned();
            }
        }

        Position::AttributeValue { name, .. } => {
            let values = HX_ATTRIBUTE_VALUES.get()?.get(&name)?;
            return Some(values.clone());
        }
    };

    None
}

pub fn espx_hover(text_params: TextDocumentPositionParams) -> Option<EspxCompletion> {
    let result = crate::tree_sitter::get_position_from_lsp_completion(text_params.clone())?;
    debug!("handle_hover result: {:?}", result);

    match result {
        Position::AttributeName(name) => HX_TAGS
            .get()
            .expect("Why it can't get HX_TAGS?")
            .iter()
            .find(|x| x.name == name)
            .cloned(),

        Position::AttributeValue { name, .. } => HX_TAGS
            .get()
            .expect("Why it can't get HX_TAGS?")
            .iter()
            .find(|x| x.name == name)
            .cloned(),
    }
}

pub static HX_TAGS: OnceLock<Vec<EspxCompletion>> = OnceLock::new();
pub static HX_ATTRIBUTE_VALUES: OnceLock<HashMap<String, Vec<EspxCompletion>>> = OnceLock::new();

fn to_hx_completion(values: Vec<(&str, &str)>) -> Vec<EspxCompletion> {
    return values.iter().filter_map(|x| x.try_into().ok()).collect();
}

pub fn init_hx_tags() {
    _ = HX_ATTRIBUTE_VALUES.set(maplit::hashmap! {
        String::from("hx-swap") => to_hx_completion(vec![
            ("innerHTML", include_str!("./hx-swap/innerHTML.md")),
            ("outerHTML", include_str!("./hx-swap/outerHTML.md")),
            ("afterbegin", include_str!("./hx-swap/afterbegin.md")),
            ("afterend", include_str!("./hx-swap/afterend.md")),
            ("beforebegin", include_str!("./hx-swap/beforebegin.md")),
            ("beforeend", include_str!("./hx-swap/beforeend.md")),
            ("delete", include_str!("./hx-swap/delete.md")),
            ("none", include_str!("./hx-swap/none.md")),
        ]),

        String::from("hx-target") => to_hx_completion(vec![
            ("closest", include_str!("./hx-target/closest.md")),
            ("find", include_str!("./hx-target/find.md")),
            ("next", include_str!("./hx-target/next.md")),
            ("prev", include_str!("./hx-target/prev.md")),
            ("this", include_str!("./hx-target/this.md")),
        ]),

        String::from("hx-boost") => to_hx_completion(vec![
            ("true", include_str!("./hx-boost/true.md")),
            ("false", include_str!("./hx-boost/false.md")),
        ]),

        String::from("hx-disabled-elt") => to_hx_completion(vec![
            ("closest", include_str!("./hx-disabled-elt/closest.md")),
            ("this", include_str!("./hx-disabled-elt/this.md")),
        ]),

        String::from("hx-trigger") => to_hx_completion(vec![
            ("click", include_str!("./hx-trigger/click.md")),
            ("once", include_str!("./hx-trigger/once.md")),
            ("changed", include_str!("./hx-trigger/changed.md")),
            ("delay:", include_str!("./hx-trigger/delay.md")),
            ("throttle:", include_str!("./hx-trigger/throttle.md")),
            ("from:", include_str!("./hx-trigger/from.md")),
            ("target:", include_str!("./hx-trigger/target.md")),
            ("consume", include_str!("./hx-trigger/consume.md")),
            ("queue:", include_str!("./hx-trigger/queue.md")),
            ("keyup", include_str!("./hx-trigger/keyup.md")),
            ("load", include_str!("./hx-trigger/load.md")),
            ("revealed", include_str!("./hx-trigger/revealed.md")),
            ("intersect", include_str!("./hx-trigger/intersect.md")),
            ("every", include_str!("./hx-trigger/every.md")),
        ]),

        String::from("hx-ext") => to_hx_completion(vec![
            ("ajax-header", include_str!("./hx-ext/ajax-header.md")),
            ("alpine-morph", include_str!("./hx-ext/alpine-morph.md")),
            ("class-tools", include_str!("./hx-ext/class-tools.md")),
            ("client-side-templates", include_str!("./hx-ext/client-side-templates.md")),
            ("debug", include_str!("./hx-ext/debug.md")),
            ("disable-element", include_str!("./hx-ext/disable-element.md")),
            ("event-header", include_str!("./hx-ext/event-header.md")),
            ("head-support", include_str!("./hx-ext/head-support.md")),
            ("include-vals", include_str!("./hx-ext/include-vals.md")),
            ("json-enc", include_str!("./hx-ext/json-enc.md")),
            ("morph", include_str!("./hx-ext/morph.md")),
            ("loading-states", include_str!("./hx-ext/loading-states.md")),
            ("method-override", include_str!("./hx-ext/method-override.md")),
            ("morphdom-swap", include_str!("./hx-ext/morphdom-swap.md")),
            ("multi-swap", include_str!("./hx-ext/multi-swap.md")),
            ("path-deps", include_str!("./hx-ext/path-deps.md")),
            ("preload", include_str!("./hx-ext/preload.md")),
            ("remove-me", include_str!("./hx-ext/remove-me.md")),
            ("response-targets", include_str!("./hx-ext/response-targets.md")),
            ("restored", include_str!("./hx-ext/restored.md")),
            ("sse", include_str!("./hx-ext/sse.md")),
            ("ws", include_str!("./hx-ext/ws.md")),
        ]),

        String::from("hx-push-url") => to_hx_completion(vec![
            ("true", include_str!("./hx-push-url/true.md")),
            ("false", include_str!("./hx-push-url/false.md")),
        ]),

        String::from("hx-swap-oob") => to_hx_completion(vec![
            ("true", include_str!("./hx-swap-oob/true.md")),
            ("innerHTML", include_str!("./hx-swap/innerHTML.md")),
            ("outerHTML", include_str!("./hx-swap/outerHTML.md")),
            ("afterbegin", include_str!("./hx-swap/afterbegin.md")),
            ("afterend", include_str!("./hx-swap/afterend.md")),
            ("beforebegin", include_str!("./hx-swap/beforebegin.md")),
            ("beforeend", include_str!("./hx-swap/beforeend.md")),
            ("delete", include_str!("./hx-swap/delete.md")),
            ("none", include_str!("./hx-swap/none.md")),
        ]),

        String::from("hx-history") => to_hx_completion(vec![
            ("false", include_str!("./hx-history/false.md")),
        ]),

        String::from("hx-params") => to_hx_completion(vec![
            ("*", include_str!("./hx-params/star.md")),
            ("none", include_str!("./hx-params/none.md")),
            ("not", include_str!("./hx-params/not.md")),
        ]),

        String::from("hx-replace-url") => to_hx_completion(vec![
            ("true", include_str!("./hx-replace-url/true.md")),
            ("false", include_str!("./hx-replace-url/false.md")),
        ]),

        String::from("hx-sync") => to_hx_completion(vec![
            ("drop", include_str!("./hx-sync/drop.md")),
            ("abort", include_str!("./hx-sync/abort.md")),
            ("replace", include_str!("./hx-sync/replace.md")),
            ("queue", include_str!("./hx-sync/queue.md")),
        ])
    });

    _ = HX_TAGS.set(to_hx_completion(vec![
        // ("hx-boost", include_str!("./attributes/hx-boost.md")),
        ("hx-boost", &block_prompt("Say hi")),
        ("hx-delete", include_str!("./attributes/hx-delete.md")),
        ("hx-get", include_str!("./attributes/hx-get.md")),
        ("hx-include", include_str!("./attributes/hx-include.md")),
        ("hx-patch", include_str!("./attributes/hx-patch.md")),
        ("hx-post", include_str!("./attributes/hx-post.md")),
        ("hx-put", include_str!("./attributes/hx-put.md")),
        ("hx-swap", include_str!("./attributes/hx-swap.md")),
        ("hx-target", include_str!("./attributes/hx-target.md")),
        ("hx-trigger", include_str!("./attributes/hx-trigger.md")),
        ("hx-vals", include_str!("./attributes/hx-vals.md")),
        ("hx-push-url", include_str!("./attributes/hx-push-url.md")),
        ("hx-select", include_str!("./attributes/hx-select.md")),
        ("hx-ext", include_str!("./attributes/hx-ext.md")),
        ("hx-on", include_str!("./attributes/hx-on.md")),
        (
            "hx-select-oob",
            include_str!("./attributes/hx-select-oob.md"),
        ),
        ("hx-swap-oob", include_str!("./attributes/hx-swap-oob.md")),
        ("hx-confirm", include_str!("./attributes/hx-confirm.md")),
        ("hx-disable", include_str!("./attributes/hx-disable.md")),
        (
            "hx-disabled-elt",
            include_str!("./attributes/hx-disabled-elt.md"),
        ),
        ("hx-encoding", include_str!("./attributes/hx-encoding.md")),
        ("hx-headers", include_str!("./attributes/hx-headers.md")),
        ("hx-history", include_str!("./attributes/hx-history.md")),
        (
            "hx-history-elt",
            include_str!("./attributes/hx-history-elt.md"),
        ),
        ("hx-indicator", include_str!("./attributes/hx-indicator.md")),
        ("hx-params", include_str!("./attributes/hx-params.md")),
        ("hx-preserve", include_str!("./attributes/hx-preserve.md")),
        ("hx-prompt", include_str!("./attributes/hx-prompt.md")),
        (
            "hx-replace-url",
            include_str!("./attributes/hx-replace-url.md"),
        ),
        ("hx-request", include_str!("./attributes/hx-request.md")),
        ("hx-sync", include_str!("./attributes/hx-sync.md")),
        ("hx-validate", include_str!("./attributes/hx-validate.md")),
    ]));
}
