use leptos::{html::Select, *};

pub struct SOption {
    key: String,
    value: String,
}
impl SOption {
    pub fn new(key: &str, value: &str) -> Self {
        Self {
            key: key.to_owned(),
            value: value.to_owned(),
        }
    }
}

#[component]
pub fn TSelect(
    cx: Scope,
    id: String,
    label: String,
    node_ref: NodeRef<Select>,
    #[prop(default = false)] required: bool,
    #[prop(default = "__".to_string())] name: String,
    #[prop(default = "__".to_string())] promp: String,
    #[prop(default = vec![])] options: Vec<SOption>,
    #[prop(default = "".to_string())] value: String,
) -> impl IntoView {
    let name = if name == "__" { id.clone() } else { name };
    let promp = if promp == "__" {
        format!("=Select {}=", label.clone())
    } else {
        promp
    };

    view! {cx,
        <div class="relative z-0 w-full mb-6 group">
            <select  class="sel block py-2.5 px-0 w-full text-sm text-secondary-40 bg-transparent border-b border-secondary-80 focus:outline-none focus:ring-0 focus:border-primary-40 peer"
                id={id.clone()}
                name={name}
                required={required}
                node_ref={node_ref}
            >
                <option value="">
                    {promp}
                </option>

                { options.into_iter()
                    .map(|opt| {
                        let value_bool: bool = opt.value == value;
                        view! { cx,
                            <option value={opt.value}
                              selected={value_bool}
                            >
                                {opt.key}
                            </option>
                    }})
                    .collect::<Vec<_>>()
                }
            </select>
            <label for={id} class="peer-focus:font-medium z-10  bg-transparent absolute text-sm text-secondary-40 duration-300 transform -translate-y-6 scale-75 top-3 -z-10 origin-[0] peer-focus:left-0 peer-focus:text-primary-40 peer-0invalid:scale-100 peer-0invalid:translate-y-0 peer-focus:scale-75 peer-focus:-translate-y-6"
                class=("after:content-['*']", move ||  required)
                class=("after:ml-0.5", move ||  required)
                class=("after:text-error-40", move ||  required)
            >
                {label}
            </label>
          </div>
    }
}
