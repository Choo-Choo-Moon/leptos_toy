use leptos::prelude::*;
use leptos::ev;

#[component]
pub fn PullToRefresh<F, Fut>(
    on_refresh: F,
    children: Children,
) -> impl IntoView
where
    F: Fn() -> Fut + 'static + Send + Sync + Clone,
    Fut: std::future::Future<Output = ()> + 'static,
{
    let (pull_y, set_pull_y) = signal(0.0);
    let (is_refreshing, set_is_refreshing) = signal(false);
    let (start_y, set_start_y) = signal::<Option<f64>>(None);
    
    // Threshold for triggering refresh
    const THRESHOLD: f64 = 80.0;
    const MAX_PULL: f64 = 120.0;

    let on_touch_start = move |e: ev::TouchEvent| {
        // Only start if scrollTop is 0 (at the top)
        if let Some(window) = web_sys::window() {
             if window.scroll_y().unwrap_or(0.0) <= 0.0 {
                 if let Some(touch) = e.touches().item(0) {
                    set_start_y.set(Some(touch.client_y() as f64));
                }
             }
        }
    };

    let on_touch_move = move |e: ev::TouchEvent| {
        if is_refreshing.get() {
            return;
        }

        if let Some(start) = start_y.get() {
            if let Some(touch) = e.touches().item(0) {
                let current_y = touch.client_y() as f64;
                let diff = current_y - start;

                if diff > 0.0 {
                    // Prevent default scrolling when pulling down
                    // Note: This might interfere with normal scrolling if not handled carefully.
                    // Usually we only prevent default if we are sure it's a pull action.
                    // But prevent_default in passive listener is not allowed in modern browsers.
                    // CSS 'overscroll-behavior-y: contain' on body helps.
                    
                    // Logarithmic resistance
                    let resistance = 0.5;
                    let move_y = (diff * resistance).min(MAX_PULL);
                    set_pull_y.set(move_y);
                }
            }
        }
    };

    let on_touch_end = move |_| {
        if is_refreshing.get() {
            return;
        }
        
        let current_pull = pull_y.get();
        if current_pull >= THRESHOLD {
            set_is_refreshing.set(true);
            set_pull_y.set(THRESHOLD); // Snap to threshold position
            
            let on_refresh = on_refresh.clone();
            
            leptos::task::spawn_local(async move {
                on_refresh().await;
                // Wait a bit to show completion
                gloo_timers::future::TimeoutFuture::new(500).await;
                set_is_refreshing.set(false);
                set_pull_y.set(0.0);
                set_start_y.set(None);
            });
        } else {
            // Cancel pull
            set_pull_y.set(0.0);
            set_start_y.set(None);
        }
    };

    view! {
        <div 
            class="relative w-full h-full"
            on:touchstart=on_touch_start
            on:touchmove=on_touch_move
            on:touchend=on_touch_end
        >
            // Refresh Indicator
            <div 
                class="absolute left-0 right-0 flex justify-center items-center pointer-events-none z-20"
                style=move || {
                    let y = pull_y.get();
                    // 당기는 거리에 따라 서서히 나타남 (임계값의 20% 지점부터 나타나기 시작)
                    let opacity = ((y - 20.0) / (THRESHOLD - 20.0)).max(0.0).min(1.0);
                    
                    // 시작 위치를 약간 더 아래로 조정 (top: 10px)
                    format!(
                        "top: 10px; transform: translateY({}px); opacity: {}; transition: transform {}ms, opacity 200ms;", 
                        y - 50.0, // -50px 지점에서 시작하여 내려옴
                        opacity,
                        if start_y.get().is_none() { 300 } else { 0 }
                    )
                }
            >
                <div class="bg-white rounded-full p-2 shadow-lg border border-gray-100">
                     {move || {
                        if is_refreshing.get() {
                            view! {
                                <div class="animate-spin rounded-full h-6 w-6 border-b-2 border-blue-600"></div>
                            }.into_any()
                        } else {
                            let rotate = (pull_y.get() * 3.0) as i32;
                            view! {
                                <svg 
                                    xmlns="http://www.w3.org/2000/svg" 
                                    fill="none" 
                                    viewBox="0 0 24 24" 
                                    stroke-width="2" 
                                    stroke="currentColor" 
                                    class="w-6 h-6 text-blue-600"
                                    style=format!("transform: rotate({}deg)", rotate)
                                >
                                    <path stroke-linecap="round" stroke-linejoin="round" d="M16.023 9.348h4.992v-.001M2.985 19.644v-4.992m0 0h4.992m-4.993 0l3.181 3.183a8.25 8.25 0 0013.803-3.7M4.031 9.865a8.25 8.25 0 0113.803-3.7l3.181 3.182m0-4.991v4.99" />
                                </svg>
                            }.into_any()
                        }
                     }}
                </div>
            </div>

            // Content
            <div
                style=move || {
                    format!(
                        "transform: translateY({}px); transition: transform {}ms;", 
                        pull_y.get(),
                        if start_y.get().is_none() { 300 } else { 0 }
                    )
                }
            >
                {children()}
            </div>
        </div>
    }
}
