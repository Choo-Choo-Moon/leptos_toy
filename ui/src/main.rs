use leptos::prelude::*;

mod domain;
mod infrastructure;
mod presentation;

use crate::presentation::layouts::main_layout::MainLayout;
use crate::presentation::pages::gallery::GalleryPage;

fn main() {
    console_error_panic_hook::set_once();

    leptos::mount::mount_to_body(|| {
        view! {
            <MainLayout>
                <GalleryPage />
            </MainLayout>
        }
    })
}
