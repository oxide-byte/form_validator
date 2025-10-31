use leptos::html::*;
use leptos::prelude::*;

use validator::prelude::*;
use validator::validators::Email;

#[component]
pub fn FormError(error: ReadSignal<String>) -> impl IntoView {
    view! {
        <Show when=move || !error.get().is_empty() fallback=|| ()>
            <p role="alert" class="text-red-600 text-sm mt-1">{ error }</p>
        </Show>
    }
}

#[component]
pub fn SimpleForm() -> impl IntoView {
    let (name, set_name) = signal("Unknown".to_string());

    let (_email, set_email) = signal(String::new());
    let (email_error, set_email_error) = signal(String::new());

    let email_validator = Email;

    view! {
        <div class="p-4 space-y-4">
            <h1 class="text-4xl"> Controlled </h1>
            <h2 class="text-2xl">Hello {name}</h2>
            <p>
                Enter your name:
                <input
                    type="text"
                    on:input:target=move |ev| {
                        set_name.set(ev.target().value());
                    }
                    prop:value=name
                    class="block border rounded px-2 py-1"
                />
            </p>

            <div class="mt-6">
                <label for="email" class="block font-medium">Email</label>
                <input
                    id="email"
                    type="email"
                    class="block border rounded px-2 py-1 w-full"
                    on:input:target=move |ev| {
                        let val = ev.target().value();
                        set_email.set(val.clone());
                        match email_validator.validate(&val) {
                            Ok(_) => set_email_error.set(String::new()),
                            Err(msg) => set_email_error.set(msg.to_string()),
                        }
                    }
                />
                <FormError error=email_error />
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(SimpleForm);
}