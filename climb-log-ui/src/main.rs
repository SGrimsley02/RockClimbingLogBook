use leptos::*;

#[component]
pub fn SimpleCounterWithBuilder(initial_value: i32) -> impl IntoView {
    use leptos::html::*;

    let (value, set_value) = create_signal(initial_value);
    let clear = move |_| set_value(0);
    let decrement = move |_| set_value.update(|value| *value -= 1);
    let increment = move |_| set_value.update(|value| *value += 1);

    // the `view` macro above expands to this builder syntax
    div().child((
        button().on(ev::click, clear).child("Clear"),
        button().on(ev::click, decrement).child("-1"),
        span().child(("Value: ", value, "!")),
        button().on(ev::click, increment).child("+1")
    ))
}



fn main() {
    mount_to_body(|| view! {
        <SimpleCounterWithBuilder initial_value=3 />
    })
}