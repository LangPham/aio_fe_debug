use leptos::*;

#[component]
pub fn TButton(cx: Scope, 
    #[prop(default = "Submit".to_string())]
    label: String,
    #[prop(default = "submit".to_string())]
    ttype: String
) -> impl IntoView {
    view! {cx,
        <button
            class="text-primary-100 bg-primary-40 hover:bg-primary-50 focus:ring-4 font-medium rounded-lg w-full sm:w-auto px-5 py-2.5 text-center"
            type={ttype}
        >
            {label}
        </button>
    }
}
