use gloo::net::http::Request;
use leptos::*;
use leptos_router::Redirect;
// use leptos_meta::*;
use crate::{
    app::GlobalState,
    components::{HeaderModel, HeaderPage, Loading},
    layout::*,
    models::TUser,
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    // let (count, set_count) = create_signal(cx, 0);
    let async_data = create_resource(
        cx,
        || (),
        |_| async move {
            let res = Request::get("/api/session").send().await.unwrap();
            match res.status() {
                400 => vec![],
                _ => {
                    let user = res.json::<TUser>().await.unwrap();
                    vec![user]
                }
            }
        },
    );

    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "Home", "List all user", "New", "/user/new"),
    );
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (user, set_user) = create_slice(
        cx,
        state,
        |state| state.user.clone(),
        |state, user| {
            state.user = user;
        },
    );

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }                
            >

            {move || async_data.read(cx).map(
                |data| {
                    match data.len() {
                        1 => {
                            let user_data = data[0].clone();
                            set_user.set(user_data);
                            log::debug!("DATAHOME:::{:#?}", user.get());
                            view! {cx,
                                <div>
                                   
                                    "Hi"
                                </div>
                               
                            }
                        },
                        _ => {
                            view! {cx,
                               <div>
                                    <Redirect path="/login"/>
                               </div>
                            }    
                        }          
                    }
                }
            )}    
            </Transition>
        </Layout>
    }    

    
}
