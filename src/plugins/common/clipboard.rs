use bevy::prelude::*;
#[cfg(target_family = "wasm")]
use bevy_defer::{AsyncAccess, AsyncCommandsExtension};

#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct ClipboardResource {
    native_clipboard: arboard::Clipboard,
}

#[cfg(target_family = "wasm")]
#[derive(Resource, Default)]
pub struct ClipboardResource;

#[cfg(not(target_family = "wasm"))]
impl Default for ClipboardResource {
    fn default() -> Self {
        Self {
            native_clipboard: arboard::Clipboard::new().expect("Failed to create native clipboard"),
        }
    }
}

impl ClipboardResource {
    #[cfg(not(target_family = "wasm"))]
    fn native_write_text(&mut self, val: String) {
        self.native_clipboard
            .set_text(val)
            .expect("Failed to write text to native clipboard");
    }

    #[cfg(not(target_family = "wasm"))]
    fn native_read_text(&mut self) -> String {
        self.native_clipboard
            .get_text()
            .expect("Failed to read text from native clipboard")
    }

    #[cfg(target_family = "wasm")]
    fn wasm_get_clipboard() -> web_sys::Clipboard {
        web_sys::window()
            .expect("Failed to get window")
            .navigator()
            .clipboard()
    }

    #[cfg(target_family = "wasm")]
    async fn wasm_write_text(val: impl Into<String>) {
        let _ = wasm_bindgen_futures::JsFuture::from(
            Self::wasm_get_clipboard().write_text(&val.into()),
        )
        .await
        .expect("Failed to write text to wasm clipboard");
    }

    #[cfg(target_family = "wasm")]
    async fn wasm_read_text() -> String {
        wasm_bindgen_futures::JsFuture::from(Self::wasm_get_clipboard().read_text())
            .await
            .expect("Failed to read text from wasm clipboard")
            .as_string()
            .expect("Failed to convert wasm clipboard data to string")
    }

    /// Copies the given text to the clipboard.
    pub fn copy(&mut self, into_text: impl Into<String>) {
        let text = into_text.into();

        #[cfg(not(target_family = "wasm"))]
        {
            self.native_write_text(text);
        }

        #[cfg(target_family = "wasm")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                Self::wasm_write_text(text).await;
            });
        }
    }

    /// Pastes the text from the clipboard into the given text reference.
    #[cfg(not(target_family = "wasm"))]
    pub fn native_paste(&mut self, destination: &mut String) {
        destination.push_str(&self.native_read_text());
    }

    /// Pastes the text from the clipboard into the given text entity.
    #[cfg(target_family = "wasm")]
    pub fn wasm_paste(&mut self, commands: &mut Commands, text_entity: Entity) {
        commands.spawn_task(move || async move {
            let clipboard_text = Self::wasm_read_text().await;
            bevy_defer::fetch!(text_entity, &mut Text).get_mut(|mut t| {
                t.0.push_str(&clipboard_text);
            })
        });
    }
}

pub fn clipboard_plugin(app: &mut App) {
    #[cfg(target_family = "wasm")]
    app.add_plugins(bevy_defer::AsyncPlugin::default_settings());
    app.init_resource::<ClipboardResource>();
}
