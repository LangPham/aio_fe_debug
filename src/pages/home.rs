use gloo::net::http::Request;
use leptos::*;
use leptos_router::Redirect;
// use leptos_meta::*;
use crate::{
    
    components::{HeaderModel, HeaderPage, Loading},
    layout::*,
    
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    

    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "Home", "List all user", "", ""),
    );   
   

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }                
            >
            <div>
                                   
                "Hi"
            </div>
           
            </Transition>
        </Layout>
    }    

    
}
