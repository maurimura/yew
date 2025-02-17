use yew::prelude::*;

struct NotToString;

fn compile_fail() {
    html! { <div> };
    html! { <div><div> };
    html! { </div> };
    html! { <div><div></div> };
    html! { <div></div><div></div> };
    html! { <div></span> };
    html! { <tag-a></tag-b> };
    html! { <div></span></div> };
    html! { <img /></img> };
    html! { <div>Invalid</div> };

    html! { <input attr=1 attr=2 /> };
    html! { <input value="123" value="456" /> };
    html! { <input kind="checkbox" kind="submit" /> };
    html! { <input checked=true checked=false /> };
    html! { <input disabled=true disabled=false /> };
    html! { <option selected=true selected=false /> };
    html! { <div class="first" class="second" /> };

    html! { <input checked=1 /> };
    html! { <input disabled=1 /> };
    html! { <option selected=1 /> };
    html! { <input type=() /> };
    html! { <input value=() /> };
    html! { <a href=() /> };

    html! { <input onclick=1 /> };
    html! { <input onclick=|| () /> };
    html! { <input onclick=|a, b| () /> };
    html! { <input onclick=|a: String| () /> };

    html! { <input string=NotToString /> };
}

fn main() {}
