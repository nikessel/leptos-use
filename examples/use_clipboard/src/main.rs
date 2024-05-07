use leptos::prelude::*;
use leptos_use::docs::{demo_or_body, Note};
use leptos_use::{use_clipboard, use_permission, UseClipboardReturn};

#[component]
fn Demo() -> impl IntoView {
    let (input, set_input) = signal("".to_owned());

    let UseClipboardReturn {
        is_supported,
        text,
        copied,
        copy,
    } = use_clipboard();

    let permission_read = use_permission("clipboard-read");
    let permission_write = use_permission("clipboard-write");

    view! {
        <Show
            when=is_supported
            fallback=|| view! { <p>Your browser does not support the Clipboard API</p> }
        >
            <Note>
                Clipboard Permission:
                read <b>{move || permission_read().to_string()}</b> |
                write <b>{move || permission_write().to_string()}</b>
            </Note>
            <p>Currently copied: <code>{move || text().unwrap_or("none".to_owned())}</code></p>
            <input value=input on:input=move |e| set_input(event_target_value(&e)) type="text" />
            <button on:click={
                let copy = copy.clone();
                move |_| copy(&input())
            }>
                <Show when=copied fallback=|| "Copy">
                    Copied!
                </Show>
            </button>
        </Show>
    }
}

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to(demo_or_body(), || {
        view! { <Demo/> }
    })
}
