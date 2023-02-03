/*
YEW TUTORIAL APP by Alexander Abraham a.k.a. "Angel Dollface".
Licensed under the MIT license.
*/

// We declare
// the "lib"
// directory as a module.
mod lib;

// We use Yew's APIs.
use yew::prelude::*;

// Importing our data processing
// component.
use lib::data_cog::DataCog;

// Importing our heading
// component.
use lib::heading_cog::HeadingCog;

// Loading our components
// into a main "App" component.
#[function_component]
fn App() -> Html {
    return html! {
        <>
         <HeadingCog/>
         <div class="content">
          <DataCog/>
         </div>
        </>
    };
}

// Main entry point for 
// the Rust compiler.
fn main() {
    
    // We render the app.
    yew::Renderer::<App>::new().render();
}