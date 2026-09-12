use backend::{
    DebugState, TransparentShapeDifficulty, auto_record_lie_detector, auto_save_rune,
    debug_state_receiver, record_video, test_spin_rune, test_transparent_shape,
    test_transparent_shape_file, test_violetta,
};
use dioxus::prelude::*;
use tokio::sync::broadcast::error::RecvError;

use crate::{
    components::{
        button::{Button, ButtonStyle},
        section::Section,
    },
    i18n::{Key, use_i18n},
};

#[component]
pub fn DebugScreen() -> Element {
    let mut state = use_signal(DebugState::default);
    let mut file_input_key = use_signal(|| 0);
    let i18n = use_i18n();

    use_future(move || async move {
        let mut rx = debug_state_receiver().await;
        loop {
            let current_state = match rx.recv().await {
                Ok(state) => state,
                Err(RecvError::Closed) => break,
                Err(RecvError::Lagged(_)) => continue,
            };
            if current_state != *state.peek() {
                state.set(current_state);
            }
        }
    });

    rsx! {
        div { class: "flex flex-col h-full overflow-y-auto",
            Section { title: i18n.t(Key::DebugSection),
                div { class: "grid grid-cols-2 gap-3",
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async {
                            test_spin_rune().await;
                        },

                        {i18n.t(Key::DebugTestSpinRune)}
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async {
                            test_violetta().await;
                        },

                        {i18n.t(Key::DebugTestVioletta)}
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async {
                            test_transparent_shape(TransparentShapeDifficulty::Normal).await;
                        },

                        {i18n.t(Key::DebugTestShapeNormal)}
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async move {
                            log::info!("[UI] Test transparent shape hard clicked");
                            test_transparent_shape(TransparentShapeDifficulty::Hard).await;
                            log::info!("[UI] Test transparent shape hard completed");
                        },

                        {i18n.t(Key::DebugTestShapeHard)}
                    }
                    label {
                        class: "inline-block h-6 text-xs text-center font-medium content-center
                                px-2 bg-secondary-surface text-secondary-text cursor-pointer",
                        input {
                            key: "{file_input_key}",
                            class: "sr-only",
                            r#type: "file",
                            accept: ".mp4,video/mp4",
                            onchange: move |e: Event<FormData>| {
                                let files = e.data.files();
                                log::info!("[UI] file input onchange, {} file(s) selected", files.len());
                                if let Some(file) = files.into_iter().next() {
                                    let path = file.path();
                                    log::info!("[UI] selected file path: {:?}", path);
                                    file_input_key += 1;
                                    spawn(async move {
                                        log::info!("[UI] spawning backend call for: {:?}", path);
                                        test_transparent_shape_file(path).await;
                                        log::info!("[UI] backend call completed for transparent shape test");
                                    });
                                }
                            },
                        }
                        {i18n.t(Key::DebugTestShapeFile)}
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async move {
                            record_video(!state.peek().is_recording).await;
                        },

                        if state().is_recording {
                            {i18n.t(Key::DebugStopRecording)}
                        } else {
                            {i18n.t(Key::DebugStartRecording)}
                        }
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async move {
                            auto_save_rune(!state.peek().is_rune_auto_saving).await;
                        },

                        if state().is_rune_auto_saving {
                            {i18n.t(Key::DebugStopAutoSaveRune)}
                        } else {
                            {i18n.t(Key::DebugStartAutoSaveRune)}
                        }
                    }
                    Button {
                        style: ButtonStyle::Secondary,
                        on_click: move |_| async move {
                            let recording = state.peek().is_lie_detector_auto_recording;
                            auto_record_lie_detector(!recording).await;
                        },

                        if state().is_lie_detector_auto_recording {
                            {i18n.t(Key::DebugStopAutoRecordLieDetector)}
                        } else {
                            {i18n.t(Key::DebugStartAutoRecordLieDetector)}
                        }
                    }
                }
            }
        }
    }
}
