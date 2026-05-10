use common::ApiStatus; // On utilise le type partagé !
use console_error_panic_hook::set_once as set_panic_hook;
use gloo_net::http::Request;
use leptos::*;

#[component]
fn App() -> impl IntoView {
    // Une ressource qui appelle l'API au chargement
    let status = create_resource(
        || (),
        |_| async move {
            Request::get("/api/status")
                .send()
                .await
                .unwrap()
                .json::<ApiStatus>() // On désérialise directement vers notre type commun
                .await
                .unwrap()
        },
    );

    view! {
        <h1>"Odo.io Dashboard"</h1>
        <Transition fallback=move || view! { <p>"Chargement..."</p> }>
            {move || status.get().map(|s| view! {
                <p>"Version du serveur : " {s.version}</p>
                <p>"Statut : " {if s.online { "✅ En ligne" } else { "❌ Hors ligne" }}</p>
                <p>"Message : " {
                    s.message.clone().unwrap_or("Aucun message".to_string())
                }
                </p>
            })}
        </Transition>
    }
}

fn main() {
    // Permet d'avoir des logs d'erreurs clairs dans la console du navigateur
    set_panic_hook();

    mount_to_body(|| view! { <App /> });
}
