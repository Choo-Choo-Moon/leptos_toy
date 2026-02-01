use crate::domain::entities::photo::Photo;
use crate::presentation::view_models::gallery_vm::GalleryViewModel;
use leptos::prelude::*;

#[component]
pub fn PhotoCard(photo: Photo) -> impl IntoView {
    let vm = use_context::<GalleryViewModel>().expect("GalleryViewModel required");

    let pid = photo.id;
    let is_liked = photo.is_liked;
    let photo_for_click = photo.clone();

    view! {
        <div 
            class="relative group rounded-lg overflow-hidden shadow-lg bg-gray-800 cursor-pointer"
            style="content-visibility: auto;"
            on:click=move |_| vm.select_photo(photo_for_click.clone())
        >
            <img 
                src=photo.url 
                loading="lazy"
                decoding="async"
                class="w-full h-48 object-cover transition-transform duration-300 group-hover:scale-105" 
            />
            <div class="absolute bottom-0 left-0 right-0 p-3 bg-gradient-to-t from-black/90 via-black/50 to-transparent pt-8">
                <p class="text-white text-sm font-medium truncate">{photo.title}</p>
                <button
                    on:click=move |e| {
                        e.stop_propagation();
                        vm.toggle_like(pid);
                    }
                    class="mt-1 flex items-center gap-1 text-xs text-gray-300 hover:text-red-400 p-1 -ml-1 rounded transition-colors active:scale-95"
                >
                    {if is_liked { "❤️" } else { "🤍" }}
                    <span>{if is_liked { "Liked" } else { "Like" }}</span>
                </button>
            </div>
        </div>
    }
}
