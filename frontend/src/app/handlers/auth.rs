//! Handlers for authentication state transitions.

use crate::api::ApiService;
use crate::app::{App, Msg};
use shared_frontend::i18n::Language;
use shared_frontend::i18n::strings::{StringKey, lookup};
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use super::show_temporary_status;

impl App {
    /// Updates the authenticated flag and surfaces a localised status banner.
    pub fn handle_set_authenticated(
        &mut self,
        ctx: &Context<Self>,
        auth: bool,
        show_notification: bool,
    ) -> bool {
        self.authenticated = auth;
        let lang = Language::from_code(&self.locale_state);
        if auth {
            if show_notification {
                let pin_success = lookup(StringKey::StatusPinSuccess, lang).to_string();
                show_temporary_status(ctx, &pin_success, "success");
            }

            let link = ctx.link().clone();
            spawn_local(async move {
                let _ = ApiService::get_leaderboard("Alpha").await;
                let _ = link;
            });
        } else {
            if show_notification {
                let logout_msg = lookup(StringKey::StatusLogout, lang).to_string();
                show_temporary_status(ctx, &logout_msg, "success");
            }
        }
        true
    }

    /// Fires the logout request and, on success, demotes the user to the
    /// login screen.
    pub fn handle_logout(&mut self, ctx: &Context<Self>) -> bool {
        let link = ctx.link().clone();
        spawn_local(async move {
            if ApiService::logout().await.is_ok() {
                link.send_message(Msg::SetAuthenticated {
                    auth: false,
                    show_notification: true,
                });
            }
        });
        false
    }
}
