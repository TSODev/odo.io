use leptos::*;
use leptos_router::A; // Import spécifique

#[component]
pub fn SignupPage() -> impl IntoView {
    let (name, set_name) = create_signal(String::new());
    let (email, set_email) = create_signal(String::new());
    let (password, set_password) = create_signal(String::new());

    view! {
        <div class="auth-page">
            <div class="auth-card">
                <h2>"Rejoindre Odo.io"</h2>
                <form on:submit=move |ev| ev.prevent_default()>
                    <div class="input-group">
                        <label>"Nom Complet"</label>
                        <input type="text"
                            on:input=move |ev| set_name.set(event_target_value(&ev))
                            prop:value=name
                        />
                    </div>
                    <div class="input-group">
                        <label>"Email Professionnel"</label>
                        <input type="email"
                            on:input=move |ev| set_email.set(event_target_value(&ev))
                            prop:value=email
                        />
                    </div>
                    <div class="input-group">
                        <label>"Mot de passe"</label>
                        <input type="password"
                            on:input=move |ev| set_password.set(event_target_value(&ev))
                            prop:value=password
                        />
                    </div>
                    <button type="submit" class="btn-auth signup">"Créer mon compte"</button>
                </form>
                <p class="auth-footer">
                    "Déjà inscrit ? " <A href="/login">"Se connecter"</A>
                </p>
            </div>
        </div>
    }
}
