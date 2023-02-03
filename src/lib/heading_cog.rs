/*
YEW TUTORIAL APP by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We use Yew's APIs.
use yew::prelude::*;

// The functional component to display
// a heading.
#[function_component]
pub fn HeadingCog() -> Html {
    return html!{
        <>
         <h1>{ "YEW TUTORIAL APP" }</h1>
        </>
    };
}