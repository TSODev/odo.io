// Empêche l'ouverture d'une console de commande sur Windows en mode release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tauri::Builder::default()
        .run(tauri::generate_context!())
        .expect("erreur lors du lancement de l'application tauri");
}