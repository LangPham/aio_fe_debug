use leptos::*;

#[component]
pub fn TButton(cx: Scope, #[prop(default = "Submit".to_string())] label: String) -> impl IntoView {
    view! {cx,
        <button
            class="text-primary-100 bg-primary-40 hover:bg-primary-50 focus:ring-4 font-medium rounded-lg text-sm w-full sm:w-auto px-5 py-2.5 text-center"
            ttype="submit"
        >
            {label}
        </button>
    }
}
