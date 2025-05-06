use leptos::prelude::*;
use leptos::web_sys;

fn handle_input<T: std::str::FromStr>(ev: web_sys::Event) -> Option<T> {
    event_target_value(&ev).parse::<T>().ok()
}

#[component]
pub fn SimpleCalculator() -> impl IntoView {
    let costPerShot = RwSignal::new(0.0);

    view! {
        <div>
            <input
                type="number"
                step="0.01"
                value=move || costPerShot.get().to_string()
                on:input=move |ev| {handle_input::<f64>(ev).map(|val| costPerShot.set(val));}
                
            />
            <p>
                "You entered: "
                {move || format!("{:.2}", costPerShot.get())}
            </p>
        </div>
    }
}
