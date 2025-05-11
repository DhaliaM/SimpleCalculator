use leptos::prelude::*;
use leptos::web_sys;

fn handle_input<T: std::str::FromStr>(ev: web_sys::Event) -> Option<T> {
    event_target_value(&ev).parse::<T>().ok()
}

#[component]
pub fn SimpleCalculator() -> impl IntoView {
    let stat_box_style = "border: 1px solid gray; padding: 4px 8px; margin-left: 8px; display: inline-block;";
    console_error_panic_hook::set_once();

    let costPerShot = RwSignal::new(0.0);
    let efficiency = RwSignal::new(0.0);
    let dps = RwSignal::new(0.0);
    let ttValue = RwSignal::new(0.0);
    let usesPerMin = RwSignal::new(0);
    let maxUses = RwSignal::new(0);
    let misses = RwSignal::new(0.0);

    let looterProf = RwSignal::new(0);
    let tax = RwSignal::new(5);

    let shots_per_hour = Memo::new(move |_| {
        let uses = usesPerMin.get();
        let reload_time = if uses == 0 { 1.0 } else { 60.0 / uses as f64 };
        3600.0 / reload_time
    });
    let costPerSingleRun = Memo::new(move |_| {
        shots_per_hour.get() * costPerShot.get() / 100.0 - ttValue.get()
        });

    view! {
        <h3>Weapon stats</h3>
        <ul>
            <li>
                <div>
                    <label>Efficiency:</label>
                    <input
                        type="number"
                        step="0.01"
                        value=move || efficiency.get().to_string()
                        on:input=move |ev| {
                            handle_input::<f64>(ev).map(|val| efficiency.set(val));
                        }
                    />
                </div>
            </li>
            <li>
                <div>
                    <label>CostPerShot:</label>
                    <input
                        type="number"
                        step="0.01"
                        value=move || costPerShot.get().to_string()
                        on:input=move |ev| {
                            handle_input::<f64>(ev).map(|val| costPerShot.set(val));
                        }
                    />
                    <label>"You entered: " {move || format!("{:.2}", costPerShot.get())}</label>
                </div>
            </li>
            <li>
                <div>
                    <label>DPS:</label>
                    <input
                        type="number"
                        step="0.01"
                        value=move || dps.get().to_string()
                        on:input=move |ev| {
                            handle_input::<f64>(ev).map(|val| dps.set(val));
                        }
                    />
                </div>
            </li>
            <li>
                <div>
                    <label>TT value:</label>
                    <input
                        type="number"
                        step="0.01"
                        value=move || ttValue.get().to_string()
                        on:input=move |ev| {
                            handle_input::<f64>(ev).map(|val| ttValue.set(val));
                        }
                    />
                </div>
            </li>
            <li>
                <div>
                    <label>uses/min:</label>
                    <input
                        type="number"
                        value=move || usesPerMin.get().to_string()
                        on:input=move |ev| {
                            handle_input::<i32>(ev).map(|val| usesPerMin.set(val));
                        }
                    />
                    <label>
                        "reload in s: "
                        {move || {
                            let u = usesPerMin.get();
                            if u == 0 {
                                "0".to_string()
                            } else {
                                format!("{:.2}", 60.0 / u as f64).to_string()
                            }
                        }}
                    </label>
                </div>
            </li>
            <li>
                <div>
                    <label>max uses:</label>
                    <input
                        type="number"
                        value=move || maxUses.get().to_string()
                        on:input=move |ev| {
                            handle_input::<i32>(ev).map(|val| maxUses.set(val));
                        }
                    />
                </div>
            </li>
            <li>
                <div>
                    <label>Misses in percent:</label>
                    <input
                        type="number"
                        step="0.01"
                        value=move || misses.get().to_string()
                        on:input=move |ev| {
                            handle_input::<f64>(ev).map(|val| misses.set(val));
                        }
                    />
                </div>
            </li>
        </ul>
        <h3>Looter</h3>
        <ul>
            <li>
                <div>
                    <label>Looter profession:</label>
                    <input
                        type="number"
                        value=move || looterProf.get().to_string()
                        on:input=move |ev| {
                            handle_input::<i32>(ev).map(|val| looterProf.set(val));
                        }
                    />
                </div>
            </li>
            <li>
                <div>
                    <label>Tax:</label>
                    <input
                        type="number"
                        value=move || tax.get().to_string()
                        on:input=move |ev| {
                            handle_input::<i32>(ev).map(|val| tax.set(val));
                        }
                    />
                </div>
            </li>
        </ul>
        <h3>Calculations for a 500hp mob and 10 skill level</h3>
        <ul>
            <li>
                <label>
                    Shots per 1h/10% run:
                    <span style="border: 1px solid gray; padding: 4px 8px; margin-left: 8px; display: inline-block;">
                        {move || format!("{:.2}", shots_per_hour.get())}
                    </span>
                </label>
            </li>
            <li>
                <label>
                    Ammunition cost per run:
                    <span style=stat_box_style>
                        {move || format!("{:.2}", costPerSingleRun.get())}
                    </span>Ped
                </label>
            </li>
            <li>
                <label>
                    Loss per single run:
                    <span style=stat_box_style>
                        {move || {
                            let tax_value = tax.get() as f64;
                            let tax_effect = if tax_value < 5.0 {
                                5.0 - tax_value
                            } else {
                                -(tax_value - 5.0)
                            };
                            let final_efficiency = 86.0 + 0.07 * efficiency.get()
                                + 0.07 * looterProf.get() as f64 + tax_effect;
                            let cost = costPerSingleRun.get();
                            let result = cost - (cost * (final_efficiency / 100.0));
                            format!("{:.2}", result)
                        }}
                    </span>
                </label>
            </li>
        </ul>
    }
}
