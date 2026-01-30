use leptos::prelude::*;

fn main() {
    leptos::mount::mount_to_body(|| {
        view! { <div> <p>"Hello, world!"</p>
        <h1 class="text-3xl font-bold text-red-600 underline bg-yellow-200 p-4">
          Tailwind Test!
        </h1>
        </div> }
    })
}
