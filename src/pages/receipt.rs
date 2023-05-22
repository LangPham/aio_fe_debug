use crate::app::GlobalState;
use crate::components::*;
use crate::layout::*;
use gloo::net::http::Request;
use leptos::*;
use leptos::ev::Event;
use leptos::ev::SubmitEvent;
use leptos::html::Input;
use leptos_router::*;

use std::rc::Rc;
use std::str::FromStr;
// #[derive(Params, PartialEq)]
// struct PageParams {
//     page: String
// }


#[component]
pub fn PageReceipt(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
        let (t_info, set_t_info) = create_slice(
            cx,
            state,
            |state| state.t_info.clone(),
            |state, value| {
                state.t_info = value;
            },
        );

    let (page_current, set_page_current) = create_signal(cx, 1);
    let (branch_current, set_branch_current) = create_signal(cx, String::default());
    let (ref_current, set_ref_current) = create_signal(cx, String::default());
    let search = use_query_map(cx);
    let page = search.with(|search| search.get("page").cloned().unwrap_or_default());
    
    let page: u64 = FromStr::from_str(&page).unwrap_or_default();
    set_page_current.clone().set(page);

    log::debug!("PAGE::::{:#?}", page);
    let async_data = create_resource(
        cx,
        move || (page_current.get(), ref_current.get(), branch_current.get()),        
        |(page_current, ref_current, branch_current)| async move {         
            let page_query = format!("?page={}&filter[ref_id]={}&filter[branch_code]={}", page_current, ref_current, branch_current);
            Request::get(&format!("/api/receipt{}", page_query))
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
        HeaderModel::new(cx, "Receipt", "List all receipt", "Home", "/"),
    );

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }
            >
                {move || async_data.read(cx).map(
                    |data| {
                        let list_data = data["entries"].as_array().unwrap();
                        let page = &data["page"];
                        
                        let def_column = [
                            DefCol::new("ID", "id", "number"),
                            DefCol::new("Branch code", "data_iportal.data.school.branch_code", "string"),
                            DefCol::new("RefId", "data_iportal.data.so_phieu_thu", "string"),
                            DefCol::new("Sync Ok", "is_on_bravo", "bool"),
                            DefCol::new("Updated At", "updated_at", "string"),
                            // DefCol::new("Username", "username", "string"),
                        ].to_vec();


                        let read = Rc::new(move |receipt: serde_json::Value| {
                            // let id = user["id"].as_i64().unwrap();
                            log::debug!("{:#?}", receipt);
                            let info = serde_json::to_string_pretty(&receipt).unwrap();
                            set_t_info.set(info);
                        });

                        view! {cx,
                            <Filter setter_branch=set_branch_current setter_ref=set_ref_current branch_current=branch_current ref_current=ref_current/>
                            <Pagination page=page.clone() setter=set_page_current loaded=true/>                            
                            <TTable data=list_data.to_vec() columns=def_column actions=vec![(read, "View".to_owned())] /> 
                        }
                    }
                )}
            </Transition>
        </Layout>
    }
}

#[component]
pub fn Filter(cx: Scope, setter_branch: WriteSignal<String>, setter_ref: WriteSignal<String>, branch_current: ReadSignal<String>, ref_current: ReadSignal<String>) -> impl IntoView {
    let input_ref_id: NodeRef<Input> = create_node_ref(cx);
    let input_branch_code: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
            ev.prevent_default();
    
            let ref_id = input_ref_id.get().expect("<input> to exist").value();
            let branch_code = input_branch_code.get().expect("<input> to exist").value();
            // let page = ref_id.parse::<u64>().unwrap();
            // let navigate = use_navigate(cx);
            // let _ = navigate(format!("/receipt?page={}", ref_id).as_str(), Default::default());
            
            setter_branch.set(branch_code.trim().to_string());
            setter_ref.set(ref_id.trim().to_string());
            
    };
    let on_reset = move |ev: Event| {
        ev.prevent_default();

        // let ref_id = input_ref_id.get().expect("<input> to exist").value();
        // let branch_code = input_branch_code.get().expect("<input> to exist").value();
        // let page = ref_id.parse::<u64>().unwrap();
        // let navigate = use_navigate(cx);
        // let _ = navigate(format!("/receipt?page={}", ref_id).as_str(), Default::default());
        
        setter_branch.set("".to_string());
        setter_ref.set("".to_string());
        
};
    view! {cx,
        <form on:submit=on_submit
            on:reset=on_reset
            class="border p-2 my-4 rounded-md"
        >
            <div class="flex gap-4">
                <TInput id="ref_id".to_string()  label="RefId".to_string() node_ref=input_ref_id value=ref_current.get_untracked()/>            
                <TInput id="branch_code".to_string()  label="Branch code".to_string() node_ref=input_branch_code value=branch_current.get_untracked()/>
            </div>
            <div class="flex gap-4 justify-center">
                <TButton label="Filter".to_string() />
                <TButton label="Clear".to_string() ttype="reset".to_string()/>
            </div>
            
        </form>
    }
}


#[component]
pub fn Pagination(cx: Scope, page: serde_json::Value, setter: WriteSignal<u64>, loaded: bool) -> impl IntoView {
    let (loading, set_loading) = create_signal(cx, true);
    let entries = page["entries"].as_u64().unwrap();
    let total = page["total"].as_u64().unwrap();
    let page = page["number"].as_u64().unwrap();
    let page_n = if page == total {total} else {page + 1};
    let page_p = if page == 1 {1} else {page-1};
    set_loading.set(!loaded) ;
    view! {cx,
    <Suspense
            fallback=move || view! { cx, <Loading/> }
        >
        <div class="flex justify-between bg-secondary-80 py-2 relative animate1-pulse">
            { 
                if loading.get() {
                    view! {cx,
                        <div class="absolute  bg-error-40 h-full w-16 -my-2 animate-loadbar"></div>
                    }
                }                
                 else {
                    view! {cx,
                        <div class="absolute"></div>
                    }
                }

            }
            
            <div>
                <A href={format!("?page={}", page_p)}
                    class="bg-primary-40 text-primary-100 p-2 transition-all hover:bg-primary-50"
                    on:click=move |_| {
                        set_loading.set(true);
                        setter.set(page_p)                        
                    }
                >
                "Pre"
                </A>
            </div>
            <div>"Page:"{page}"/"{total}" | Total:" {entries}</div>
            <div>
                <A href={format!("?page={}", page_n)}
                    class="bg-primary-40 text-primary-100 p-2 transition-all hover:bg-primary-50"
                    on:click=move |_| {
                        set_loading.set(true);
                        setter.set(page_n)                        
                    }
                >
                "Next"
                </A>
            </div>
        </div>
    </Suspense>
    }
}

