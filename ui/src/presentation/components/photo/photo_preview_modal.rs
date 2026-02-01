use crate::domain::entities::photo::Photo;
use leptos::prelude::*;

#[component]
pub fn PhotoPreviewModal(
    photo: Photo, 
    on_close: Callback<()>
) -> impl IntoView {
    // 닫기 이벤트 핸들러 (배경 클릭 시 닫기)
    let on_backdrop_click = move |_| on_close.run(());

    // 내부 컨텐츠 클릭 시 이벤트 전파 방지 (모달 닫힘 방지)
    let on_content_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
    };

    view! {
        // Backdrop (Fixed overlay)
        <div 
            class="fixed inset-0 z-50 flex items-center justify-center bg-black/90 p-4 transition-opacity duration-300"
            on:click=on_backdrop_click
        >
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