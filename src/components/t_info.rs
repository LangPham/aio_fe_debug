use leptos::*;

use crate::app::GlobalState;

#[component]
pub fn TInfo(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (t_info, set_t_info) = create_slice(
        cx,
        state,
        |state| state.t_info.clone(),
        |state, value| {
            state.t_info = value;
        },
    );


    view! { cx,
      <Show
        when=move || t_info.get() != ""
        fallback=|_cx| view! { cx, "" }
      >
      <div
          class="z-50 w-full h-full fixed top-0 bg-secondary-40 bg-opacity-80 flex justify-center items-center "
          on:click=move |_| {
              set_t_info.set("".to_string());
          }
          >
          <div class="block bg-secondary-99 text-primay-0 w-auto max-h-full h-auto rounded divide-y divide-double w-min-64 overflow-y-auto">
              <div class="p-4 relative bg-primary-40 text-primary-100"> "INFO!!!" </div>
              <div class="p-4 "> <pre>{t_info.get()}</pre></div>
          </div>
      </div>
      </Show>
    }
}
