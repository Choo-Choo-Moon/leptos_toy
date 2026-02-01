use crate::domain::entities::photo::Photo;
use crate::presentation::view_models::gallery_vm::GalleryViewModel;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn PhotoPreviewModal(
    photo: Photo, 
    on_close: Callback<()>
) -> impl IntoView {
    let vm = use_context::<GalleryViewModel>().expect("GalleryViewModel required");

    // 닫기 이벤트 핸들러 (배경 클릭 시 닫기)
    let on_backdrop_click = move |_| on_close.run(());

    // 내부 컨텐츠 클릭 시 이벤트 전파 방지 (모달 닫힘 방지)
    let on_content_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
    };

    // 키보드 이벤트 핸들러 (ArrowLeft, ArrowRight)
    Effect::new(move |_| {
        let handle_keydown = move |e: web_sys::KeyboardEvent| {
            match e.key().as_str() {
                "ArrowLeft" => vm.prev_photo(),
                "ArrowRight" => vm.next_photo(),
                "Escape" => on_close.run(()),
                _ => {}
            }
        };

        let closure = Closure::wrap(Box::new(handle_keydown) as Box<dyn FnMut(web_sys::KeyboardEvent)>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        }

        move || {
            if let Some(window) = web_sys::window() {
                let _ = window.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
        }
    });

    view! {
        // Backdrop (Fixed overlay)
        <div 
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/90 p-4 transition-opacity duration-300"
            on:click=on_backdrop_click
        >
            // Navigation Buttons (Left)
            <button 
                class="absolute left-4 top-1/2 -translate-y-1/2 z-50 p-3 bg-black/50 hover:bg-black/70 rounded-full text-white transition focus:outline-none"
                on:click=move |e| {
                    e.stop_propagation();
                    vm.prev_photo();
                }
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                </svg>
            </button>

            // Navigation Buttons (Right)
             <button 
                class="absolute right-4 top-1/2 -translate-y-1/2 z-50 p-3 bg-black/50 hover:bg-black/70 rounded-full text-white transition focus:outline-none"
                on:click=move |e| {
                    e.stop_propagation();
                    vm.next_photo();
                }
            >
                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                </svg>
            </button>

            // Modal Content
            <div 
                class="relative max-w-5xl w-full max-h-[90vh] bg-gray-900 rounded-lg shadow-2xl overflow-hidden flex flex-col md:flex-row"
                on:click=on_content_click
            >
                // Image Section
                <div class="flex-1 bg-black flex items-center justify-center overflow-hidden">
                    <img 
                        src=photo.url 
                        alt=photo.title.clone()
                        class="max-w-full max-h-[70vh] md:max-h-[90vh] object-contain"
                    />
                </div>

                // Details Section
                <div class="w-full md:w-80 p-6 flex flex-col border-l border-gray-800 text-gray-200">
                    <h2 class="text-2xl font-bold mb-4 text-white">{photo.title}</h2>
                    
                    <div class="space-y-4 flex-1">
                        <div class="flex items-center space-x-2 text-sm text-gray-400">
                            <span>"ID:"</span>
                            <span class="font-mono text-xs">{photo.id.to_string()}</span>
                        </div>
                        
                        <div class="p-4 bg-gray-800 rounded-lg">
                            <p class="text-sm">"Metadata information will appear here."</p>
                        </div>
                    </div>

                    // Actions
                    <div class="mt-6 pt-6 border-t border-gray-800 flex justify-end">
                         <button 
                            class="px-4 py-2 bg-gray-700 hover:bg-gray-600 text-white rounded transition"
                            on:click=move |_| on_close.run(())
                        >
                            "Close"
                        </button>
                    </div>
                </div>
            </div>
        </div>
    }
}