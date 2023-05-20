use crate::components::*;
use crate::layout::*;
use gloo::net::http::Request;
use leptos::*;


#[component]
pub fn PageCustomer(cx: Scope) -> impl IntoView {
    
    let async_data = create_resource(
        cx,
        || (),
        |_| async move {
            Request::get("/api/customer")
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
        HeaderModel::new(cx, "Customer", "List all customer", "Home", "/"),
    );

    view! { cx,
        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }
            >
                {move || async_data.read(cx).map(
                    |data| {
                        let list_customer = data["entries"].as_array().unwrap();


                        let def_column = [
                            DefCol::new("Mã khách hàng", "customer_id", "number"),
                            DefCol::new("Mã trường", "branch_code", "string"),
                            DefCol::new("Họ tên", "data_iportal.customer.full_name", "string"),
                            DefCol::new("Đã lên bravo", "is_on_bravo", "bool"),
                            DefCol::new("Update", "updated_at", "string"),
                            // DefCol::new("Username", "username", "string"),
                            
                        ].to_vec();
                        view! {cx, <TTable data=list_customer.to_vec() columns=def_column actions=vec![] /> }
                    }
                )}
            </Transition>
        </Layout>
    }
}

