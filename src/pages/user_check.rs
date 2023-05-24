use crate::components::*;
use crate::{layout::*};
use gloo::net::http::Request;
use leptos::ev::Event;
use leptos::{
    ev::SubmitEvent,
    html::{Input, Select},
    *,
};


#[component]
pub fn ListUserCheck(cx: Scope) -> impl IntoView {    
    let (post_current, set_post_current) = create_signal(cx, "1".to_string());



    let async_data = create_resource(
        cx,
        move || (post_current.get()),
        |post| async move {           
            Request::get(&format!("https://jsonplaceholder.typicode.com/comments?postId={}", post))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap()
        },
    );

    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "User", "List all user", "New", "/user/new"),
    );

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }
            >
                {move || async_data.read(cx).map(
                    |data| {
                        let list_data = data.as_array().unwrap();

                        let def_column = [
                            DefCol::new("ID", "id", "number"),
                            DefCol::new("Email", "email", "string"),
                            DefCol::new("Name", "name", "string"),
                            DefCol::new("Body", "body", "string"),                            
                        ].to_vec();
                        // view! {cx, <TTable data=data columns=def_column actions=vec![(edit, "Edit".to_owned()), (toggle_active, "Status".to_owned()), (delete, "Delete".to_owned())] /> }
                        view! {cx,
                            <div>
                                <Filter setter_post=set_post_current post_current=post_current />
                                // <Pagination page=page.clone() setter=set_page_current loaded=true/>
                                <TTable data=list_data.to_vec() columns=def_column actions=vec![] />
                            </div>
                        }
                    }
                )}
            </Transition>
        </Layout>
    }
}

#[component]
pub fn Filter(
    cx: Scope,
    setter_post: WriteSignal<String>,    
    post_current: ReadSignal<String>,    
) -> impl IntoView {    
    let input_post_code: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();        
        let post_code = input_post_code.get().expect("<input> to exist").value();
        setter_post.set(post_code.trim().to_string());
        
    };
    let on_reset = move |ev: Event| {
        ev.prevent_default();
        setter_post.set("".to_string());        
    };
    view! {cx,
        <form on:submit=on_submit
            on:reset=on_reset
            class="border p-2 my-4 rounded-md"
        >
            <div class="flex gap-4">                
                <TInput id="post_id".to_string()  label="Post ID".to_string() node_ref=input_post_code value=post_current.get_untracked()/>
            </div>
            <div class="flex gap-4 justify-center">
                <TButton label="Filter".to_string() />
                <TButton label="Clear".to_string() ttype="reset".to_string()/>
            </div>
        </form>
    }
}