use leptos::*;

use crate::{app::GlobalState, models::TUser};

#[component]
pub fn Logout(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
        let (_user, set_user) = create_slice(
            cx,
            state,
            |state| state.user.clone(),
            |state, user| {
                state.user = user;
            },
        );
    set_user.set(TUser::default());

    view! { cx,
        <div class="flex items-center justify-center h-screen">
            <div class="p-1 rounded shadow-lg bg-gradient-to-r from-primary-40 via-error-40 to-primary-80">
              <div class="flex flex-col items-center p-4 space-y-2 bg-primary-100">
                <svg xmlns="http://www.w3.org/2000/svg" class="text-primary-40 w-28 h-28" fill="none" viewBox="0 0 24 24" stroke="currentColor" stroke-width="1">
                  <path stroke-linecap="round" stroke-linejoin="round" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
                </svg>
                <h1 class="text-4xl font-bold font-extrabold text-primary-40 bg-clip-text bg-gradient-to-r from-blue-500 to-purple-500">"Thank You !"</h1>
                <p>"Thank you for your interest! Check your email for a link to the guide."</p>
                // <a
                //   class="inline-flex items-center px-4 py-2 text-primary-100 bg-primary-40 border border-indigo-600 rounded rounded-full hover:bg-indigo-700 focus:outline-none focus:ring">
                //   <svg xmlns="http://www.w3.org/2000/svg" class="w-3 h-3 mr-2" fill="none" viewBox="0 0 24 24"
                //     stroke="currentColor" stroke-width="2">
                //     <path stroke-linecap="round" stroke-linejoin="round" d="M7 16l-4-4m0 0l4-4m-4 4h18" />
                //   </svg>
                //   <span class="text-sm font-medium">
                //     "Home"
                //   </span>
                // </a>
              </div>
            </div>
          </div>
    }
}
