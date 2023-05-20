use crate::components::*;
use crate::{app::GlobalState, layout::*};
use gloo::net::http::Request;
use leptos::{
    ev::SubmitEvent,
    html::{Input, Select},
    *,
};
use leptos_router::*;
use std::rc::Rc;

#[component]
pub fn ListUser(cx: Scope) -> impl IntoView {
    let (loading, set_loading) = create_signal(cx, false);
    let async_data = create_resource(
        cx,
        move || loading.get(),
        |_| async move {
            Request::get("/api/user")
                .send()
                .await
                .unwrap()
                .json::<Vec<serde_json::Value>>()
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
                        let edit = Rc::new(move |user: serde_json::Value| {
                            let navigate = use_navigate(cx);
                            let id = &user["id"]                            ;
                            let _ = navigate(format!("/user/{}/edit", id).as_str(), Default::default());
                        });
                        let toggle_active = Rc::new(move |user: serde_json::Value| {
                            let id = user["id"].as_i64().unwrap();
                            spawn_local(async move {
                                let change = if user["status"].as_str().unwrap() == "Active" {
                                    serde_json::json!({
                                        "status": "Draft",
                                    })
                                } else {
                                    serde_json::json!({
                                        "status": "Active",
                                    })
                                };
                                let resp = Request::put(&format!("/api/user/{}/status", id))
                                    .json(&change)
                                    .expect("REASON")
                                    .send()
                                    .await
                                    .unwrap();
                                match resp.status() {
                                    200 => {
                                        set_loading.update(|n| *n = !*n);
                                    }
                                    _ => {
                                        let result_json = resp.json::<serde_json::Value>().await.unwrap();
                                        let e = result_json["error"].as_str().unwrap().to_string();
                                        log::error!("{:#?}", e)
                                        // set_t_err.set(e);
                                    }
                                }
                            });
                        });

                        let delete = Rc::new(move |user: serde_json::Value| {
                            let id = user["id"].as_i64().unwrap();
                            spawn_local(async move {
                                let change =
                                    serde_json::json!({
                                        "status": "Deleted",
                                    });

                                let resp = Request::put(&format!("/api/user/{}/status", id))
                                    .json(&change)
                                    .expect("REASON")
                                    .send()
                                    .await
                                    .unwrap();
                                match resp.status() {
                                    200 => {
                                        set_loading.update(|n| *n = !*n);
                                    }
                                    _ => {
                                        let result_json = resp.json::<serde_json::Value>().await.unwrap();
                                        let e = result_json["error"].as_str().unwrap().to_string();
                                        log::error!("{:#?}", e)
                                        // set_t_err.set(e);
                                    }
                                }
                            });
                        });

                        let def_column = [
                            DefCol::new("ID", "id", "number"),
                            DefCol::new("Username", "username", "string"),
                            DefCol::new("Name", "name", "string"),
                            DefCol::new("Role", "role", "string"),
                            DefCol::new("Status", "status", "string"),
                        ].to_vec();
                        view! {cx, <TTable data=data columns=def_column actions=vec![(edit, "Edit".to_owned()), (toggle_active, "Status".to_owned()), (delete, "Delete".to_owned())] /> }
                    }
                )}
            </Transition>
        </Layout>
    }
}

#[component]
pub fn NewUser(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (_t_err, set_t_err) = create_slice(
        cx,
        state,
        |state| state.t_error.clone(),
        |state, value| {
            state.t_error = value;
        },
    );

    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "New User", "Form for new user", "Back", "/user"),
    );
    let input_username: NodeRef<Input> = create_node_ref(cx);
    let input_name: NodeRef<Input> = create_node_ref(cx);
    let select_role: NodeRef<Select> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = input_username.get().expect("<input> to exist").value();
        let name = input_name.get().expect("<input> to exist").value();
        let role = select_role.get().expect("<select> to exist").value();

        log::debug!("FORM:::{:#?}-{:#?}", username, name);
        spawn_local(async move {
            let body = serde_json::json!({
                "username": username,
                "name": name,
                "status": "Active",
                "role": role
            });

            let resp = Request::post(&format!("/api/user"))
                .json(&body)
                .expect("REASON")
                .send()
                .await
                .unwrap();

            match resp.status() {
                200 => {
                    let navigate = use_navigate(cx);
                    let _ = navigate(format!("/user").as_str(), Default::default());
                }
                _ => {
                    let result_json = resp.json::<serde_json::Value>().await.unwrap();
                    let e = result_json["error"].as_str().unwrap().to_string();
                    set_t_err.set(e);
                }
            }
        });
    };
    view! { cx,

        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }
            >
                <form  on:submit=on_submit>
                    <TInput id="username".to_string() label="Username".to_string() node_ref=input_username required=true/>
                    <TInput id="name".to_string() label="Name".to_string() node_ref=input_name required=true/>
                    <TSelect id="name".to_string() label="Role".to_string() required=true options=vec![SOption::new("Admin", "Admin"), SOption::new("Guest", "Guest")] node_ref=select_role/>
                    <TButton/>
                </form>
            </Transition>
        </Layout>
    }
}

#[component]
pub fn EditUser(cx: Scope) -> impl IntoView {
    let (id, set_id) = create_signal(cx, String::default());

    let params = use_params_map(cx);
    let idparams = params.with(|params| params.get("id").cloned().unwrap_or_default());
    set_id.set(idparams);

    let async_data = create_resource(
        cx,
        move || id.get(),
        |value| async move {
            Request::get(&format!("/api/user/{}", value))
                .send()
                .await
                .unwrap()
                .json::<serde_json::Value>()
                .await
                .unwrap()
        },
    );

    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (_t_err, set_t_err) = create_slice(
        cx,
        state,
        |state| state.t_error.clone(),
        |state, value| {
            state.t_error = value;
        },
    );

    let (header, _) = create_signal(
        cx,
        HeaderModel::new(cx, "Edit User", "Form for edit user", "Back", "/user"),
    );
    let input_username: NodeRef<Input> = create_node_ref(cx);
    let input_name: NodeRef<Input> = create_node_ref(cx);
    let select_role: NodeRef<Select> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let username = input_username.get().expect("<input> to exist").value();
        let name = input_name.get().expect("<input> to exist").value();
        let role = select_role.get().expect("<select> to exist").value();
        log::debug!("FORM:::{:#?}-{:#?}", username, name);
        spawn_local(async move {
            let body = serde_json::json!({
                "username": username,
                "name": name,
                "status": "Active",
                "role": role

            });

            let resp = Request::put(&format!("/api/user/{}", id.get()))
                .json(&body)
                .expect("REASON")
                .send()
                .await
                .unwrap();
            log::debug!("RESPONE:::{:#?}", resp.status());
            match resp.status() {
                200 => {
                    // window().location().replace("/user").unwrap();
                    let navigate = use_navigate(cx);
                    let _ = navigate(format!("/user").as_str(), Default::default());
                }
                _ => {
                    let result_json = resp.json::<serde_json::Value>().await.unwrap();
                    let e = result_json["error"].as_str().unwrap().to_string();
                    set_t_err.set(e);
                }
            }
        });
    };
    view! { cx,

        <Layout>
            <HeaderPage header=header />
            <Transition
                fallback=move || view! { cx, <Loading/> }
            >
                {move || async_data.read(cx).map(
                    |data| {
                        log::debug!("DATA:::{:#?}", data);
                        let value_username = data["username"].as_str().unwrap().to_string();
                        let value_name = data["name"].as_str().unwrap().to_string();
                        let value_role = data["role"].as_str().unwrap().to_string();
                        view! {cx,
                            <form  on:submit=on_submit>
                                <TInput id="username".to_string()  label="Username".to_string() node_ref=input_username value=value_username/>
                                <TInput id="name".to_string()  label="Name".to_string() node_ref=input_name value=value_name/>
                                <TSelect id="name".to_string() label="Role".to_string() required=true value=value_role options=vec![SOption::new("Admin", "Admin"), SOption::new("Guest", "Guest")] node_ref=select_role/>
                                <TButton/>
                            </form>
                        }
                    }
                )}
            </Transition>
        </Layout>
    }
}
