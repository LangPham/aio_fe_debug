use crate::{models::TUser, pages::*};
use gloo::net::http::Request;
use leptos::*;
use leptos_router::*;

#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub theme_menu_open: bool,
    pub t_error: String,
    pub t_info: String,
    pub user: TUser,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, GlobalState::default());
    provide_context(cx, state);
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

    {move || async_data.read(cx).map(
        |data| {
            match data.len() {
                1 => {
                    let user_data = data[0].clone();
                    let (_user, set_user) = create_slice(
                        cx,
                        state,
                        |state| state.user.clone(),
                        |state, user| {
                            state.user = user;
                        },
                    );
                    set_user.set(user_data);                    
                    view! { cx,
                        <TRouter />                        
                    }
                },
                _ => {
                    view! { cx,
                        <TRouter />                        
                    }
                }          
            }
        }
    )}
    
}


#[component]
fn TRouter(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
            <Routes>
                <Route path="/" view=|cx| view! { cx, <Home/> }/>
                <Route path="/user" view=|cx| view! { cx,  <ListUser/>}/>
                <Route path="/user/new" view=|cx| view! { cx,  <NewUser/>}/>
                <Route path="/user/:id/edit" view=|cx| view! { cx,  <EditUser/>}/>
                <Route path="/login" view=|cx| view! { cx,  <Login/>}/>
                <Route path="/logout" view=|cx| view! { cx,  <Logout/>}/>
  
                <Route path="/customer" view=|cx| view! { cx,  <PageCustomer/>}/>
                <Route path="/receipt" view=|cx| view! { cx,  <PageReceipt/>}/>
                <Route path="/payment" view=|cx| view! { cx,  <PagePayment/>}/>
            </Routes>          
        </Router>
      }
}

