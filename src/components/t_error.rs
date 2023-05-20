use leptos::*;

use crate::app::GlobalState;

#[component]
pub fn TError(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (t_err, set_t_err) = create_slice(
        cx,
        state,
        |state| state.t_error.clone(),
        |state, value| {
            state.t_error = value;
        },
    );


    view! { cx,
      <Show
        when=move || t_err.get() != ""
        fallback=|_cx| view! { cx, "" }
      >
      <div
          class="z-50 w-full h-full fixed top-0 bg-secondary-40 bg-opacity-80 flex justify-center items-center"
          on:click=move |_| {
              set_t_err.set("".to_string());
          }
          >
          <div class="block bg-error-40 text-error-100 w-auto h-auto rounded divide-y divide-double w-min-64">
              <div class="p-4 relative"> "ERROR!!!" </div>
              <div class="p-4"> <pre>{t_err.get()}</pre></div>
          </div>
      </div>
      </Show>
    }
}
