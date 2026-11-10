use gpui::IntoElement;
use ui::prelude::*;

use crate::ApiKeysWithoutProviders;

pub struct AgentPanelOnboarding;

impl AgentPanelOnboarding {
    pub fn new(_cx: &mut Context<Self>) -> Self {
        Self
    }
}

impl Render for AgentPanelOnboarding {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Zed AI requires a Zed account, and account sign-in is disabled in Kon.
        ApiKeysWithoutProviders::new()
    }
}
