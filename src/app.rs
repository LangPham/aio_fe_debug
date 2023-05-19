use crate::{models::TUser, pages::*};
use leptos::*;
use leptos_router::*;

#[derive(Default, Clone, Debug)]
pub struct GlobalState {
    pub theme_menu_open: bool,
    pub t_error: String,
    pub user: TUser,
}

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    let state = create_rw_signal(cx, GlobalState::default());
    provide_context(cx, state);
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
          </Routes>
        //   <div>"Ngoai router copy right"</div>
      </Router>
    }
}
