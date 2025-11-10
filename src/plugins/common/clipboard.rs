use bevy::prelude::*;
#[cfg(target_family = "wasm")]
use bevy_defer::{AsyncAccess, AsyncCommandsExtension};
#[cfg(target_family = "wasm")]
use std::cell::{Cell, RefCell};
#[cfg(target_family = "wasm")]
use wasm_bindgen::closure::Closure;
#[cfg(target_family = "wasm")]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(target_family = "wasm")]
use wasm_bindgen_futures::JsFuture;
#[cfg(target_family = "wasm")]
use web_sys::{console, DomException, Event};

#[cfg(not(target_family = "wasm"))]
#[derive(Resource)]
pub struct ClipboardResource {
    native_clipboard: arboard::Clipboard,
}

#[cfg(target_family = "wasm")]
#[derive(Resource, Default)]
pub struct ClipboardResource;

#[cfg(target_family = "wasm")]
thread_local! {
    static PENDING_COPY_TEXT: RefCell<Option<String>> = const { RefCell::new(None) };
    static POINTER_LISTENER_REGISTERED: Cell<bool> = const { Cell::new(false) };
    static POINTER_UP_HANDLER: RefCell<Option<Closure<dyn FnMut(Event)>>> = RefCell::new(None);
    static TOUCH_END_HANDLER: RefCell<Option<Closure<dyn FnMut(Event)>>> = RefCell::new(None);
}

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
    fn wasm_get_clipboard() -> Result<web_sys::Clipboard, JsValue> {
        let window = web_sys::window().ok_or_else(|| JsValue::from_str("Failed to get window"))?;
        Ok(window.navigator().clipboard())
    }

    #[cfg(target_family = "wasm")]
    fn ensure_wasm_event_handlers() {
        POINTER_LISTENER_REGISTERED.with(|registered| {
            if registered.get() {
                return;
            }
            registered.set(true);

            let Some(window) = web_sys::window() else {
                console::error_1(&"window unavailable; clipboard pointer fallback disabled".into());
                return;
            };

            let pointer_up = Closure::wrap(Box::new(|_: Event| {
                ClipboardResource::flush_pending_copy();
            }) as Box<dyn FnMut(Event)>);

            if let Err(error) = window
                .add_event_listener_with_callback("pointerup", pointer_up.as_ref().unchecked_ref())
            {
                console::error_2(&"failed to install pointerup handler: ".into(), &error);
            }

            POINTER_UP_HANDLER.with(|slot| {
                slot.borrow_mut().replace(pointer_up);
            });

            let touch_end = Closure::wrap(Box::new(|_: Event| {
                ClipboardResource::flush_pending_copy();
            }) as Box<dyn FnMut(Event)>);

            if let Err(error) = window
                .add_event_listener_with_callback("touchend", touch_end.as_ref().unchecked_ref())
            {
                console::error_2(&"failed to install touchend handler: ".into(), &error);
            }

            TOUCH_END_HANDLER.with(|slot| {
                slot.borrow_mut().replace(touch_end);
            });
        });
    }

    #[cfg(target_family = "wasm")]
    fn flush_pending_copy() {
        PENDING_COPY_TEXT.with(|pending| {
            if let Some(text) = pending.borrow_mut().take() {
                Self::start_clipboard_write(text, false);
            }
        });
    }

    #[cfg(target_family = "wasm")]
    fn start_clipboard_write(text: String, clear_pending_on_success: bool) {
        let clipboard = match Self::wasm_get_clipboard() {
            Ok(clipboard) => clipboard,
            Err(error) => {
                console::error_2(&"navigator.clipboard unavailable: ".into(), &error);
                return;
            }
        };

        let promise = clipboard.write_text(&text);
        wasm_bindgen_futures::spawn_local(async move {
            match JsFuture::from(promise).await {
                Ok(_) => {
                    if clear_pending_on_success {
                        PENDING_COPY_TEXT.with(|pending| {
                            let mut pending = pending.borrow_mut();
                            if pending.as_ref().is_some_and(|p| p == &text) {
                                pending.take();
                            }
                        });
                    }
                }
                Err(error) => {
                    let is_not_allowed = error
                        .dyn_ref::<DomException>()
                        .is_some_and(|dom| dom.name() == "NotAllowedError");

                    if clear_pending_on_success && is_not_allowed {
                        // Expected while we wait for the pointer/touch listener to fire.
                        return;
                    }

                    console::error_2(&"clipboard.writeText failed: ".into(), &error);
                }
            }
        });
    }

    #[cfg(target_family = "wasm")]
    fn wasm_write_text(val: impl Into<String>) {
        Self::ensure_wasm_event_handlers();

        let text = val.into();
        PENDING_COPY_TEXT.with(|pending| {
            *pending.borrow_mut() = Some(text.clone());
        });

        Self::start_clipboard_write(text, true);
    }

    #[cfg(target_family = "wasm")]
    async fn wasm_read_text() -> String {
        let clipboard = Self::wasm_get_clipboard().expect("Failed to access wasm clipboard");
        JsFuture::from(clipboard.read_text())
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
            self.native_write_text(text.clone());
        }

        #[cfg(target_family = "wasm")]
        {
            Self::wasm_write_text(text);
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
    {
        app.add_plugins(bevy_defer::AsyncPlugin::default_settings());
        ClipboardResource::ensure_wasm_event_handlers();
    }
    app.init_resource::<ClipboardResource>();
}
