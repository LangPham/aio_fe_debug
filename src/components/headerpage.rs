use leptos::*;
use leptos_router::*;

#[derive(Clone)]
pub struct HeaderModel {
    pub title: ReadSignal<String>,
    pub describe: ReadSignal<String>,
    pub label: ReadSignal<String>,
    pub to: ReadSignal<String>,
}

impl HeaderModel {
    pub fn new(cx: Scope, title: &str, describe: &str, label: &str, to: &str) -> Self {
        let (title, _) = create_signal(cx, title.to_string());
        let (describe, _) = create_signal(cx, describe.to_string());
        let (label, _) = create_signal(cx, label.to_string());
        let (to, _) = create_signal(cx, to.to_string());
        Self {
            title,
            describe,
            label,
            to,
        }
    }
}

#[component]
pub fn HeaderPage(cx: Scope, header: ReadSignal<HeaderModel>) -> impl IntoView {
    // log::debug!("{:#?}",action);
    view! { cx,
        <div class="sm:flex sm:items-center pt-4 pb-8">
            <div  class="sm:flex-auto">
                <h1 class="text-base font-semibold leading-6 text-gray-900">
                    { header.get().title.get()}
                </h1>
                <p class="mt-2 text-sm text-gray-700"> {header.get().describe.get()} </p>
            </div>
            <div class="mt-4 sm:ml-16 sm:mt-0 sm:flex-none">
                <A  class="text-primary-100 bg-primary-40 hover:bg-primary-50 transition-all rounded-md p-2 text-base"
                    href={header.get().to.get()}
                    >
                    {header.get().label.get()}
                </A>
            </div>
        </div>
    }
}
