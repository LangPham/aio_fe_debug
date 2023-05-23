use leptos::*;

#[component]
pub fn Loading(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex justify-between bg-secondary-80 py-2 relative animate1-pulse">
            <div class="absolute  bg-error-40 h-full w-16 -my-2 animate-loadbar"></div>
        </div>
    }
}
