use leptos::prelude::*;

#[component]
pub fn MainLayout(children: Children) -> impl IntoView {
    view! {
        <div class="h-dvh w-full flex flex-col bg-gray-50 overflow-hidden">
            // Fixed Header (Title Bar)
            <header class="flex-none bg-white shadow-sm z-10 border-b border-gray-200">
                <div class="w-full px-4 h-14 flex items-center justify-between">
                    <div class="flex items-center space-x-4">
                        <h1 class="text-lg font-bold text-gray-800 tracking-tight">"PhotoVault"</h1>
                    </div>
                    <div class="flex items-center space-x-2">
                        // Placeholder for right-aligned actions
                    </div>
                </div>
            </header>

            // Scrollable Main Content
            <main class="flex-1 overflow-y-auto overflow-x-hidden">
                 {children()}
            </main>
        </div>
    }
}
