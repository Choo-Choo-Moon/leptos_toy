use crate::presentation::components::common::pull_to_refresh::PullToRefresh;
use crate::presentation::components::photo::photo_card::PhotoCard;
use crate::presentation::components::photo::photo_preview_modal::PhotoPreviewModal;
use crate::presentation::view_models::gallery_vm::GalleryViewModel;
use leptos::html;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use web_sys::{IntersectionObserver, IntersectionObserverEntry, IntersectionObserverInit};

#[component]
pub fn GalleryPage() -> impl IntoView {
    // 1. ViewModel 생성 (Provider Pattern)
    let vm = GalleryViewModel::new();
    provide_context(vm);

    // Sentinel Element Reference
    let load_trigger = NodeRef::<html::Div>::new();

    // 2. 초기 데이터 로드 & Intersection Observer 설정
    Effect::new(move |_| {
        // 초기 로드
        vm.load_more();
    });

    // PopState(뒤로가기) 이벤트 리스너
    Effect::new(move |_| {
        let handle_popstate = move |_| {
            vm.sync_on_popstate();
        };

        let closure = Closure::wrap(Box::new(handle_popstate) as Box<dyn FnMut(web_sys::Event)>);
        
        if let Some(window) = web_sys::window() {
            let _ = window.add_event_listener_with_callback("popstate", closure.as_ref().unchecked_ref());
        }

        // Cleanup function (Leptos Effects can return a cleanup closure, but here we keep it simple or let it leak for page lifetime)
        // In a real app, use `leptos-use` or proper cleanup.
        // For this page which is likely always mounted or main, it is acceptable.
        closure.forget(); 
    });

    // Observer 연결 Effect
    Effect::new(move |prev_observer: Option<Option<IntersectionObserver>>| {
        // Clean up previous observer if exists
        if let Some(Some(observer)) = prev_observer {
            observer.disconnect();
        }

        let trigger_el = load_trigger.get();
        if let Some(el) = trigger_el {
            // Callback Closure
            let callback = move |entries: Vec<IntersectionObserverEntry>,
                                 _: IntersectionObserver| {
                if entries.first().is_some_and(|entry| entry.is_intersecting()) {
                    vm.load_more();
                }
            };

            // Rust Closure -> JS Function
            let closure = Closure::wrap(Box::new(callback)
                as Box<dyn FnMut(Vec<IntersectionObserverEntry>, IntersectionObserver)>);
            let js_callback = closure.as_ref().unchecked_ref();

            // Create Observer
            let options = IntersectionObserverInit::new();
            options.set_root_margin("200px"); // 미리 로드하기 위해 200px 여유

            let observer = IntersectionObserver::new_with_options(js_callback, &options)
                .expect("Failed to create IntersectionObserver");

            observer.observe(&el);

            // Keep closure alive properly (Leptos handles cleanup usually but manual management is safer here or utilize leptos-use in real world)
            closure.forget();

            return Some(observer);
        }
        None
    });

    view! {
        <PullToRefresh on_refresh=move || async move { vm.refresh().await }>
            <div class="w-full md:container md:mx-auto px-2 md:px-4 py-4 md:py-8">
                <h1 class="text-2xl md:text-3xl font-bold text-gray-800 mb-4 md:mb-6 px-1">"My Gallery"</h1>

                {move || {
                    let state = vm.state.get();

                    view! {
                        <div>
                                                    // Modal (Overlay)
                                                    <PhotoPreviewModal 
                                                        on_close=Callback::new(move |_| vm.close_preview()) 
                                                    />
                                                        <div class="grid grid-cols-2 sm:grid-cols-3 md:grid-cols-4 lg:grid-cols-5 xl:grid-cols-6 gap-2 md:gap-4">
                                {state.photos.into_iter()
                                    .map(|photo| view! { <PhotoCard photo=photo /> })
                                    .collect_view()}
                            </div>

                            // Load More Trigger Sentinel
                            <div
                                node_ref=load_trigger
                                class="h-20 flex justify-center items-center mt-8"
                            >
                                {if state.is_loading {
                                    view! {
                                        <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-gray-900"></div>
                                    }.into_any()
                                } else if !state.has_more {
                                    view! { <span class="text-gray-500">"No more photos"</span> }.into_any()
                                } else {
                                    view! { <span class="text-transparent">"Loading trigger"</span> }.into_any()
                                }}
                            </div>
                        </div>
                    }
                }}
            </div>
        </PullToRefresh>
    }
}
