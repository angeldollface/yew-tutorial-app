/*
YEW TUTORIAL APP by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

// We need this to work with
// events.
use wasm_bindgen::JsCast;

// Importing our
// password tools.
use super::utils::*;

// We need this to
// copy the generated password
// into the clipboard.
use yew_hooks::prelude::*;

// We need this to capture event
// results.
use web_sys::EventTarget;

// We need this to interact
// with the HTML "input"
// element.
use web_sys::HtmlInputElement;

// The functional component to accept a password
// length and render it as an emoji.
#[function_component]
pub fn DataCog() -> Html {

    // We instantiate a stateful clipboard handler.
    let clipboard = use_clipboard();

    // We instantiate a stateful container for our generated password.
    let result: UseStateHandle<String> = use_state(|| "Your password will appear here.".to_owned());

    // We instantiate a stateful container for our password length.
    let length: UseStateHandle<String> = use_state(|| "Password length goes here.".to_owned());

    // A handler to constantly update our password length's state.
    let onchange = {

        // Cloning our password length so that we can mutate it in the DOM.
        let length_cloned: UseStateHandle<String> = length.clone();

        // Closure to handle changes. (Inline function.)
        Callback::from(
            move |event: Event| {

                // Getting the target's properties.
                let target: EventTarget = event.target().unwrap();

                // Unwrapping it into an HTML Input element.
                let input: HtmlInputElement = target.unchecked_into::<HtmlInputElement>();

                // Setting the typed password length to the input's value.
                length_cloned.set(input.value());
            }
        )
    };

    // A handler to render our password when the button is clicked.
    let onclick = {

        // Cloning our result so that we can mutate it in the DOM.
        let result_clone = result.clone();

        // Cloning our clipboard handler so that we can mutate it and write to it.
        let clipboard_clone = clipboard.clone();

        // Closure to render the password on a button press. (Inline function.)
        move |_| {

            // We check if the user's input is valid. If it is,
            // we render the password and copy it to the clipboard.
            if is_int(&length) {
                let password: String = make_pwd(&to_int(&length.to_string()));
                let return_msg: String = format!("Copied and rendered: {}", &password.to_string());
                clipboard_clone.write_text(password);
                result_clone.set(return_msg);
            }

            // If it isn't, we return an error message.
            else {
                let return_msg: String = String::from("Invalid number sequence!");
                result_clone.set(return_msg);
            }
        }
    };

    // Inserting our input field, button,
    // and feedback element into the DOM.
    return html!{
        <>
         <input type="text" {onchange}/>
         <button {onclick}>{ "GENERATE" }</button>
         <p class="feedback">
          { format!("{}", &result.clone().to_string()) }
         </p>
        </>
    };
}