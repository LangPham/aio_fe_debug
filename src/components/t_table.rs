use leptos::*;
use std::rc::Rc;
use std::str::FromStr;
#[derive(Debug, Clone)]
pub struct DefCol {
    pub header: String,
    pub key: String,
    pub show: String,
}

impl DefCol {
    pub fn value(&self, row: serde_json::Value) -> String {
        // Get value
        let value = get_value(&self.key, &row);
        // Show value
        if value.is_null() {
            return "".to_string();
        }
        match self.show.as_str() {
            "string" => value.as_str().unwrap().to_string(),
            "number" => value.to_string(),
            "bool" => value.to_string(),
            _ => "None".to_owned(),
        }
    }
    pub fn new(header: &str, key: &str, show: &str) -> Self {
        Self {
            header: header.to_owned(),
            key: key.to_owned(),
            show: show.to_owned(),
        }
    }
}

fn get_value(key: &str, value: &serde_json::Value) -> serde_json::Value {
    let keys: Vec<_> = key.split(".").collect();
    if keys.len() == 1 {
        let n: Result<usize, _> = FromStr::from_str(keys[0]);
        let is_array = value.is_array();
        match (n, is_array) {
            (Ok(key), true) => {
                let new_value = &value[key];
                return new_value.clone();
            }
            (Ok(_), _) => {
                let new_value = &value[keys[0]];
                return new_value.clone();
            }
            (Err(_), _) => {
                let new_value = &value[keys[0]];
                return new_value.clone();
            }
        }
    } else {
        let len = keys.len();
        dbg!(len);
        let key_array = &keys[1..len];
        dbg!(key_array);
        let key = key_array.join(".");
        dbg!("Key:::", &key);
        let value = &value[keys[0]];
        dbg!("Value:::", &value);
        return get_value(key.as_str(), value);
    }
}

#[component]
pub fn TTable(
    cx: Scope,
    columns: Vec<DefCol>,
    data: Vec<serde_json::Value>,
    actions: Vec<(Rc<dyn Fn(serde_json::Value)>, String)>,
) -> impl IntoView {
    let (row_current, set_row_current) = create_signal(cx, 0);

    view! { cx,
        <table class="w-full divide-y divide-secondary-50 table-auto">
                <thead>
                    <tr>
                    { columns.to_vec().into_iter()
                        .map(|item| view! { cx,
                            <th scope="col" class="p-4 text-left  font-semibold text-gray-900">{item.header}</th>
                        })
                        .collect::<Vec<_>>()
                    }
                    {
                        if actions.len() >0 {
                            view!{cx,
                                <th scope="col" class="p-4 text-right font-semibold text-gray-900">
                                    <span class="sr-only1">{"Action"}</span>
                                </th>
                            }.into_view(cx)
                        } else {
                            view!{cx, }.into_view(cx)
                        }
                    }

                    </tr>
                </thead>
                <tbody class="divide-y divide-secondary-80 bg-white">
                    {
                        data.iter().enumerate().map(|(index, row)| {
                            let action_vec = actions.to_vec();
                            view! { cx,
                                <tr class="hover:bg-secondary-80 transition-all"
                                    class=("bg-secondary-90", move || index+1 == row_current.get())
                                    on:click= move |_| {set_row_current.set(index+1)}
                                    // data-index={index+1}
                                >
                                    { columns.to_vec().into_iter()
                                        .map(|column| view! { cx,
                                            <td class="px-3 py-2 text-gray-500">{column.value(row.clone())}</td>
                                        })
                                        .collect::<Vec<_>>()
                                    }
                                    {if actions.len() >0 {
                                        view!{cx,
                                            <td class="px-3 py-2 text-gray-500 text-right">
                                                { action_vec.into_iter()
                                                    .map(|act| {
                                                        let data = row.clone();
                                                        view! { cx,
                                                        <span
                                                            class="text-primary-40 hover:text-primary-70 cursor-pointer transition-all ml-2"
                                                            on:click=move |_| {
                                                                (act.0)(data.clone());
                                                            }
                                                        >
                                                            {act.1}
                                                        </span>
                                                    }})
                                                    .collect::<Vec<_>>()
                                                }

                                            </td>
                                        }.into_view(cx)
                                    } else {
                                        view!{cx, }.into_view(cx)
                                    }}

                                </tr>
                            }
                        }).collect::<Vec<_>>()
                    }
                </tbody>
        </table>
    }
}
