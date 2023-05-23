use leptos::*;
use crate::{    
    components::{HeaderModel, HeaderPage, Loading},
    layout::*,    
};

#[component]
pub fn Home(cx: Scope) -> impl IntoView {
    
    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "Home", "Home page", "", ""),
    );   

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }                
            >
            <h1>                                   
                "Welcome AIO App !!!"
            </h1>
            <p>
                "Use left menu to view functions!"
            </p>

            <p>
                "To use old interface click " <a href="/admin" rel="external" class="text-primary-40">"HERE"</a>
            </p>
           
            </Transition>
        </Layout>
    }
}
