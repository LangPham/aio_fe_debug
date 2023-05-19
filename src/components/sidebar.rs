use crate::app::GlobalState;
use leptos::*;
use leptos_icons::*;
use leptos_router::*;

#[component]
pub fn Sidebar(cx: Scope) -> impl IntoView {
    let state = use_context::<RwSignal<GlobalState>>(cx).expect("state to have been provided");
    let (menu_open, set_menu_open) = create_slice(
        cx,
        state,
        |state| state.theme_menu_open,
        |state, open| {
            state.theme_menu_open = open;
        },
    );

    view! { cx,
        <aside  class="flex flex-col bg-primary-40 transition-all z-10"
            class=("w-64", move || menu_open.get() == true)
            class=("w-14", move || menu_open.get() == false)
            >

            <div class="sticky top-0 flex">
                <div class="mt-2 flex-1">
                    <div class="h-12 relative transition-all"
                        class=("w-64", move || menu_open.get() == true)
                        class=("w-14", move || menu_open.get() == false)

                    >
                        <div class="absolute right-0 h-14 w-14 flex justify-center items-center text-primary-100 py-2 px-2"
                            class=("rotate-180", move || menu_open.get() == false)                            
                            on:click=move |_| {
                                set_menu_open.set( !menu_open.get());
                            }
                        >
                            <Icon  width="2em" height="2em" icon=BsIcon::BsArrowBarLeft />
                        </div>
                    </div>
                    <div class="mt-2 flex-1 ">
                        <nav class="space-y-4 px-2">
                            <A href="/"
                                class="text-primary-100 bg-primary-40 hover:bg-primary-50 transition-all group flex items-center rounded-md px-2 py-2 text-base font-medium gap-4 aria-[current=page]:bg-primary-50"
                                exact=true
                                >
                                <Icon class="mx-2" icon=BsIcon::BsHouseFill/>
                                "Home"
                            </A>
                            <A  class="text-primary-100 bg-primary-40 hover:bg-primary-50 transition-all group flex items-center rounded-md px-2 py-2 text-base font-medium gap-4 aria-[current=page]:bg-primary-50"
                                exact=true
                                href="/user" >
                                <Icon class="mx-2" icon=BsIcon::BsPeopleFill/>
                                "Users"
                            </A>

                            <A  class="text-primary-100 bg-primary-40 hover:bg-primary-50 transition-all group flex items-center rounded-md px-2 py-2 text-base font-medium gap-4 aria-[current=page]:bg-primary-50"
                                exact=true
                                href="/customer" >
                                <Icon class="mx-2" icon=BsIcon::BsPeopleFill/>
                                "Customer"
                            </A>

                            <A  class="text-primary-100 bg-primary-40 hover:bg-primary-50 transition-all group flex items-center rounded-md px-2 py-2 text-base font-medium gap-4 aria-[current=page]:bg-primary-50"
                                exact=true
                                href="/receipt" >
                                <Icon class="mx-2" icon=FaIcon::FaReceiptSolid/>
                                "Receipt"
                            </A>

                        </nav>
                    </div>
                </div>
            </div>
     </aside>



    }
}
// html! {
//     <ul class="item-list">
//         { items.iter().collect::<Html>() }
//     </ul> {
//     nav.into_iter().map(|name| {
//         html!{<div key={name.0}>{ format!("Module {} has role {}!",name.0, name.1) }</div>}
//     }).collect::<Html>()
// }
// };
