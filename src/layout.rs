use crate::{components::*, app::GlobalState};
use leptos::*;
use leptos_router::Redirect;

#[component]
pub fn LayoutPublic(cx: Scope, children: Children) -> impl IntoView {
    view! { cx,
        <div class="flex min-h-screen">
            <Sidebar />
            <div class="flex flex-1 flex-col bg-primary-100 min-h-screen z-20">
                // Topbar
                <div class="px-4 sm:px-6 lg:px-8">
                    {children(cx)}
                </div>
            </div>
             <TError />

          </div>
    }
}


#[component]
pub fn Layout(cx: Scope, children: Children) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
        let (user, _set_user) = create_slice(
            cx,
            state,
            |state| state.user.clone(),
            |state, user| {
                state.user = user;
            },
        );
        log::debug!("DATAHOME:::{:#?}", user.get());
    match user.get().id {
        // Todo: change to 0
        -1 => view! {cx,
            <Redirect path="/login"/>
        }.into_view(cx),
        _ => view! { cx,
            <div class="flex min-h-screen">
                <Sidebar />
                <div class="flex flex-1 flex-col bg-primary-100 min-h-screen z-20">
                    <TTopbar/>
                    <div class="px-4 sm:px-6 lg:px-8">
                        {children(cx)}
                    </div>
                </div>
                 <TError />
    
              </div>
        }.into_view(cx)
    }     
    
}