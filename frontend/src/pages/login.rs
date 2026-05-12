use leptos::*;
use leptos_router::A; // Import spécifique

#[component]
pub fn LoginPage() -> impl IntoView {
    let (_email, _set_email) = create_signal(String::new());
    let (_password, _set_password) = create_signal(String::new());

    view! {
            <div class="bg-blue-200 flex items-center justify-center min-h-screen p-4">
                <div class="auth-card animate-fade-in">
                    <h2 class="text-3xl font-black text-center mb-8 tracking-tight">&quot;Laisse moi entrer !&quot;</h2>
                    <form class="space-y-6" on:submit=move |ev| ev.prevent_default()>
                        <div>
                            <label class="form-label">"Email"</label>
                            <input type="email" class="form-input" placeholder="nom@entreprise.com"/>
                        </div>
                        <div>
                            <label class="form-label">"Mot de passe"</label>
                            <input type="password" class="form-input" placeholder="••••••••"/>
                        </div>
                        <button type="submit" class="btn-primary">
                            Connexion
    </button>
                    </form>
                    <p class="mt-8 text-center text-gray-500 text-sm">
                        "Pas encore de compte ? " <A href="/signup" class="text-brand-primary hover:underline font-medium">"Créer un accès"</A> </p>
                </div>
            </div>
            }
}
