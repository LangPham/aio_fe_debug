use leptos::*;

#[component]
pub fn TTopbar(cx: Scope) -> impl IntoView {
    // let navigator = use_navigator().unwrap();

    // let logout = Callback::from(move |_| navigator.push(&AnyRoute::new("/logout")));
    view! { cx,
        <aside class="flex w-full h-16 p-2 bg-white shadow justify-end items-center">
            <button
                type="button"
                class="h-8 w-8 rounded-full mr-3 bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2rounded-full ml-3 bg-white p-1 text-gray-400 hover:text-gray-500 focus:outline-none focus:ring-2 focus:ring-indigo-500 focus:ring-offset-2"
            >
                <span class="sr-only">{"Style grid"}</span>
                // <Icon icon_id={IconId::HeroiconsOutlineBuildingLibrary} />
            </button>


        </aside>
    }
}
