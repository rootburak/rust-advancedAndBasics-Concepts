use leptos::mount::mount_to_body;
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView{
    let (sayac,set_sayac) = signal(0);
    view! {
        
        <button on:click=move  |_| {*set_sayac.write() +=1 }class:red=move|| sayac.get() % 2 == 0 > "tıklama sayısı  "{sayac}</button>
        
    {move || if sayac.get() >= 5 {
        view! { <p>"more then 5!"</p> }
    } else {
        view! { <p>"continue"</p> }
    }}
    
    }
    
}
#[component]
fn App2() -> impl IntoView{
    let ogeler = vec!["wasm","yew","leptos"];
   view!{ 
    <ul>
     {ogeler.into_iter().map(|oge| view! {
        <li>{oge}</li>
     }).collect_view()}
        
    </ul>
}
}
fn main() {
    mount_to_body(App);
    mount_to_body(App2);
}