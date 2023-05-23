use leptos::*;

#[component]
pub fn Login(cx: Scope) -> impl IntoView {
    view! {cx,
        <div class="flex min-h-full flex-col justify-center py-12 sm:px-6 lg:px-8">
            <div class="sm:mx-auto sm:w-full sm:max-w-md">
              <h2 class="mt-6 text-center text-3xl font-bold tracking-tight text-gray-900">
                {"Sign in to your account"}
              </h2>
            </div>

            <div class="mt-8 sm:mx-auto sm:w-full sm:max-w-md">
              <div class="bg-white py-8 px-4 shadow sm:rounded-lg sm:px-10">
                <div class="grid grid-cols-1 gap-3">
                  <div>
                    <a
                      href="/auth/microsoft" rel="external"
                                      class="inline-flex w-full justify-center rounded-md bg-white py-2 px-4 text-gray-500 shadow-sm ring-1 ring-inset ring-gray-300 hover:bg-gray-50 focus:outline-offset-0"
                    >
                      <span class="">{"Sign in with Office 365"}</span>

                    </a>
                  </div>
                </div>
              </div>
            </div>
          </div>
    }
}
