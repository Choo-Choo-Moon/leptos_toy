use crate::presentation::view_models::gallery_vm::GalleryViewModel;
use leptos::portal::Portal;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[component]
pub fn PhotoPreviewModal(
    on_close: Callback<()>
) -> impl IntoView {
    let vm = use_context::<GalleryViewModel>().expect("GalleryViewModel required");

    // 현재 선택된 사진을 반응형으로 구독
    let photo_memo = Memo::new(move |_| {
        vm.state.get().selected_photo
    });

    // Swipe 제스처 상태
    let (drag_x, set_drag_x) = signal(0.0);
    let (start_x, set_start_x) = signal::<Option<f64>>(None);
    let (is_animating, set_is_animating) = signal(false);

    // 터치 시작
    let on_touch_start = move |e: web_sys::TouchEvent| {
        if let Some(touch) = e.touches().item(0) {
            set_start_x.set(Some(touch.client_x() as f64));
            set_is_animating.set(false); // 드래그 중에는 애니메이션 끄기 (즉각 반응)
        }
    };

    // 터치 이동
    let on_touch_move = move |e: web_sys::TouchEvent| {
        if let Some(start) = start_x.get() {
            if let Some(touch) = e.touches().item(0) {
                let current_x = touch.client_x() as f64;
                let diff = current_x - start;
                set_drag_x.set(diff);
            }
        }
    };

    // 터치 종료
    let on_touch_end = move |_| {
        if let Some(_) = start_x.get() {
            let diff = drag_x.get();
            let threshold = 50.0; // 스와이프 민감도

            if diff.abs() > threshold {
                // 임계값을 넘으면 페이지 이동
                if diff > 0.0 {
                    vm.prev_photo();
                } else {
                    leptos::task::spawn_local(async move {
                        vm.next_photo().await;
                    });
                }
            }
            
            // 상태 초기화
            set_is_animating.set(true); // 복귀 시 애니메이션 켜기 (부드럽게)
            set_drag_x.set(0.0);
            set_start_x.set(None);
        }
    };

    // 닫기 이벤트 핸들러
    let on_backdrop_click = move |_| on_close.run(());

    // 내부 컨텐츠 클릭 시 이벤트 전파 방지
    let on_content_click = move |e: web_sys::MouseEvent| {
        e.stop_propagation();
    };

    // 키보드 이벤트 핸들러
    Effect::new(move |_| {
        let handle_keydown = move |e: web_sys::KeyboardEvent| {
            match e.key().as_str() {
                "ArrowLeft" => vm.prev_photo(),
                "ArrowRight" => {
                    leptos::task::spawn_local(async move {
                        vm.next_photo().await;
                    });
                },
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
        <Portal>
            {move || {
                // photo_memo가 Some일 때만 렌더링
                if let Some(photo) = photo_memo.get() {
                    let photo_url = photo.preview_url.clone();
                    let photo_title = photo.title.clone();
                    let photo_id = photo.id.to_string();
                    
                    view! {
                        // Backdrop (Fixed overlay)
                        <div 
                            class="fixed inset-0 z-50 flex items-center justify-center bg-black/90 p-0 md:p-4 transition-opacity duration-300"
                            on:click=on_backdrop_click
                        >
                            // Navigation Buttons (Left) - Hidden on Mobile
                            <button 
                                class="hidden md:block absolute left-4 top-1/2 -translate-y-1/2 z-50 p-3 bg-black/50 hover:bg-black/70 rounded-full text-white transition focus:outline-none"
                                on:click=move |e| {
                                    e.stop_propagation();
                                    vm.prev_photo();
                                }
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M15.75 19.5L8.25 12l7.5-7.5" />
                                </svg>
                            </button>

                            // Navigation Buttons (Right) - Hidden on Mobile
                             <button 
                                class="hidden md:block absolute right-4 top-1/2 -translate-y-1/2 z-50 p-3 bg-black/50 hover:bg-black/70 rounded-full text-white transition focus:outline-none"
                                on:click=move |e| {
                                    e.stop_propagation();
                                    leptos::task::spawn_local(async move {
                                        vm.next_photo().await;
                                    });
                                }
                            >
                                <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-8 h-8">
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M8.25 4.5l7.5 7.5-7.5 7.5" />
                                </svg>
                            </button>

                            // Modal Content Container
                            <div 
                                class="relative w-full h-full md:w-full md:max-w-6xl md:h-[90vh] bg-black md:bg-gray-900 md:rounded-lg shadow-2xl overflow-hidden flex flex-col md:flex-row"
                                on:click=on_content_click
                                on:touchstart=on_touch_start
                                on:touchmove=on_touch_move
                                on:touchend=on_touch_end
                                style=move || {
                                    format!(
                                        "transform: translateX({}px); transition: {}", 
                                        drag_x.get(),
                                        if is_animating.get() { "transform 0.3s ease-out" } else { "none" }
                                    )
                                }
                            >
                                // Mobile Close Button (Top-Right Overlay)
                                <button 
                                    class="md:hidden absolute top-[calc(1.5rem+env(safe-area-inset-top))] right-6 z-50 p-2 bg-black/50 text-white rounded-full backdrop-blur-sm shadow-lg border border-white/10"
                                    on:click=move |_| on_close.run(())
                                >
                                    <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="2" stroke="currentColor" class="w-6 h-6">
                                        <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                    </svg>
                                </button>

                                // Image Section
                                <div class="flex-1 bg-black flex items-center justify-center overflow-hidden relative min-h-[50vh] min-w-[50vw] p-0 md:p-4">
                                    // Loading Spinner (Always behind, visible when image is loading or hidden)
                                    <div class="absolute inset-0 flex items-center justify-center z-0">
                                        <div class="animate-spin rounded-full h-12 w-12 border-4 border-gray-700 border-t-white"></div>
                                    </div>

                                    <img 
                                        src=photo_url
                                        alt=photo_title.clone()
                                        class=move || format!(
                                            "max-w-full max-h-full object-contain block z-10 relative transition-opacity duration-300 {}",
                                            if vm.state.get().is_loading { "opacity-0" } else { "opacity-100" }
                                        )
                                        style="min-height: 200px; min-width: 200px;" 
                                    />
                                    
                                    // Mobile Image Info Overlay (Bottom Gradient)
                                    
                                    // Mobile Image Info Overlay (Bottom Gradient)
                                    <div class="md:hidden absolute bottom-0 left-0 right-0 p-6 pb-[calc(2.5rem+env(safe-area-inset-bottom))] bg-gradient-to-t from-black/90 via-black/60 to-transparent text-white pt-24 pointer-events-none">
                                        <h2 class="text-xl font-bold truncate leading-tight">{photo_title.clone()}</h2>
                                        <p class="text-xs text-gray-300 mt-1 font-mono opacity-80">{photo_id.clone()}</p>
                                    </div>
                                </div>

                                // Desktop Details Sidebar (Hidden on Mobile)
                                <div class="hidden md:flex w-80 p-6 flex-col border-l border-gray-800 text-gray-200 bg-gray-900">
                                    <div class="flex justify-between items-start mb-6">
                                        <h2 class="text-2xl font-bold text-white leading-tight">{photo_title}</h2>
                                        <button 
                                            class="text-gray-400 hover:text-white"
                                            on:click=move |_| on_close.run(())
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
                                            </svg>
                                        </button>
                                    </div>
                                    
                                    <div class="space-y-6 flex-1 overflow-y-auto">
                                        <div class="space-y-1">
                                            <span class="text-xs font-semibold text-gray-500 uppercase tracking-wider">"Photo ID"</span>
                                            <p class="font-mono text-sm break-all">{photo_id}</p>
                                        </div>
                                        
                                        <div class="p-4 bg-gray-800 rounded-lg border border-gray-700">
                                            <h3 class="text-sm font-semibold text-gray-300 mb-2">"Information"</h3>
                                            <p class="text-sm text-gray-400">"Metadata extraction feature coming soon."</p>
                                        </div>
                                    </div>

                                    // Desktop Actions
                                    <div class="mt-6 pt-6 border-t border-gray-800 flex justify-end gap-3">
                                         <button 
                                            class="px-4 py-2 bg-gray-800 hover:bg-gray-700 text-white rounded transition border border-gray-600"
                                        >
                                            "Download"
                                        </button>
                                         <button 
                                            class="px-4 py-2 bg-blue-600 hover:bg-blue-500 text-white rounded transition"
                                        >
                                            "Share"
                                        </button>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }.into_any()
                } else {
                    view! {}.into_any()
                }
            }}
        </Portal>
    }
}
