use crate::app::GlobalState;
use crate::components::*;
use crate::layout::*;
use gloo::net::http::Request;
use leptos::*;
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
    let search = use_query_map(cx);
    let page = search.with(|search| search.get("page").cloned().unwrap_or_default());
    
    let page: u64 = FromStr::from_str(&page).unwrap_or_default();
    set_page_current.clone().set(page);

    log::debug!("PAGE::::{:#?}", page);
    let async_data = create_resource(
        cx,
        move || page_current.get(),        
        |page_current| async move {         
            let page_query = format!("?page={}", page_current);
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
                            DefCol::new("STT", "id", "number"),
                            DefCol::new("Mã trường", "data_iportal.data.school.branch_code", "string"),
                            DefCol::new("RefId", "data_iportal.data.so_phieu_thu", "string"),
                            DefCol::new("Đã lên bravo", "is_on_bravo", "bool"),
                            DefCol::new("Update", "updated_at", "string"),
                            // DefCol::new("Username", "username", "string"),
                        ].to_vec();


                        let read = Rc::new(move |receipt: serde_json::Value| {
                            // let id = user["id"].as_i64().unwrap();
                            log::debug!("{:#?}", receipt);
                            let info = serde_json::to_string_pretty(&receipt).unwrap();
                            set_t_info.set(info);
                        });




                        view! {cx,
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

