use leptos::{html::Input, *};

#[component]
pub fn TInput(
    cx: Scope,
    id: String,
    label: String,
    node_ref: NodeRef<Input>,
    #[prop(default = false)] required: bool,
    #[prop(default = "__".to_string())] name: String,
    #[prop(default = "text".to_string())] ttype: String,
    #[prop(default = "".to_string())] value: String,
) -> impl IntoView {
    let name = if name == "__" { id.clone() } else { name };
    view! {cx,
    <div class="relative z-0 w-full mb-6 group">
        <input
            required={required}
            type={ttype}
            id={id.clone()}
            name={name}
            class="block py-2.5 px-0 w-full text-sm text-secondary-40 bg-transparent border-b border-secondary-80 appearance-none focus:outline-none focus:ring-0 focus:border-primary-40 peer"
            placeholder=" "
            value={value}
            node_ref={node_ref}
        />
        <label
            for={id}
            class="peer-focus:font-medium absolute text-sm text-secondary-40 duration-300 transform -translate-y-6 scale-75 top-3 z-10 origin-[0] peer-focus:left-0 peer-focus:text-primary-40 peer-placeholder-shown:scale-100 peer-placeholder-shown:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
            class=("after:content-['*']", move ||  required)
            class=("after:ml-0.5", move ||  required)
            class=("after:text-error-40", move ||  required)
        >
            {label}
        </label>
    </div>
    }
}
