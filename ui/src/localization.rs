use backend::{
    DetectionTemplate, Localization, convert_image_to_base64, query_localization, query_template,
    save_capture_image, upsert_localization,
};
use dioxus::{html::FileData, prelude::*};
use futures_util::{StreamExt, future::OptionFuture};

use crate::{
    AppState,
    components::{
        button::{Button, ButtonStyle},
        file::FileInput,
        labeled::Labeled,
        section::Section,
    },
    i18n::{Key, use_i18n},
};

#[derive(Debug)]
enum LocalizationUpdate {
    Update(Localization),
}

#[derive(PartialEq, Clone, Copy)]
struct LocalizationContext {
    localization: Memo<Localization>,
    save_localization: Callback<Localization>,
}

#[component]
pub fn LocalizationScreen() -> Element {
    let mut localization = use_context::<AppState>().localization;
    let localization_view = use_memo(move || localization().unwrap_or_default());

    // Handles async operations for localization-related
    let coroutine = use_coroutine(
        move |mut rx: UnboundedReceiver<LocalizationUpdate>| async move {
            while let Some(message) = rx.next().await {
                match message {
                    LocalizationUpdate::Update(new_localization) => {
                        localization.set(Some(upsert_localization(new_localization).await));
                    }
                }
            }
        },
    );
    let save_localization = use_callback(move |new_localization: Localization| {
        coroutine.send(LocalizationUpdate::Update(new_localization));
    });

    use_context_provider(|| LocalizationContext {
        localization: localization_view,
        save_localization,
    });

    use_future(move || async move {
        if localization.peek().is_none() {
            localization.set(Some(query_localization().await));
        }
    });

    rsx! {
        div { class: "flex flex-col h-full overflow-y-auto",
            SectionInfo {}
            SectionPopups {}
            SectionFamiliars {}
            SectionHexa {}
            SectionOthers {}
        }
    }
}

#[component]
fn SectionInfo() -> Element {
    #[component]
    fn Header(title: &'static str) -> Element {
        rsx! {
            th { class: "text-xs text-primary-text text-left font-medium border-b border-primary-border",
                {title}
            }
        }
    }

    #[component]
    fn Data(description: &'static str, #[props(default)] rowspan: Option<usize>) -> Element {
        rsx! {
            td {
                class: "text-xs text-secondary-text border-b border-secondary-border pt-2 pr-1",
                rowspan,
                {description}
            }
        }
    }

    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::LocalizationInfo),
            table { class: "table-fixed",
                thead {
                    tr {
                        Header { title: i18n.t(Key::LocalizationHeaderSection) }
                        Header { title: i18n.t(Key::LocalizationHeaderFunction) }
                        Header { title: i18n.t(Key::LocalizationHeaderTemplates) }
                    }
                }
                tbody {
                    tr {
                        Data { description: i18n.t(Key::LocalizationSectionPopups), rowspan: 3 }
                        Data { description: i18n.t(Key::LocalizationDescUnstuck) }
                        Data { description: i18n.t(Key::LocalizationDescAllPopups) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescGoToTown) }
                        Data { description: i18n.t(Key::LocalizationDescConfirmPopup) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescRespawn) }
                        Data { description: i18n.t(Key::LocalizationDescOkNewPopup) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationSectionFamiliars), rowspan: 2 }
                        Data { description: i18n.t(Key::LocalizationDescSortFamiliar) }
                        Data { description: i18n.t(Key::LocalizationDescLevelSortButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescSaveFamiliars) }
                        Data { description: i18n.t(Key::LocalizationDescSaveButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationSectionHexa), rowspan: 4 }
                        Data { description: i18n.t(Key::LocalizationDescErdaMenu) }
                        Data { description: i18n.t(Key::LocalizationDescErdaConversionButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescBoosterExchangeMenu) }
                        Data { description: i18n.t(Key::LocalizationDescHexaBoosterButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescMaxAmount) }
                        Data { description: i18n.t(Key::LocalizationDescMaxButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescConvert) }
                        Data { description: i18n.t(Key::LocalizationDescConvertButton) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationSectionOthers), rowspan: 4 }
                        Data { description: i18n.t(Key::LocalizationDescChangeChannelMenu) }
                        Data { description: i18n.t(Key::LocalizationDescChangeChannelText) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescCashShop) }
                        Data { description: i18n.t(Key::LocalizationDescCashShopText) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescBoosterInUse) }
                        Data { description: i18n.t(Key::LocalizationDescTimerText) }
                    }
                    tr {
                        Data { description: i18n.t(Key::LocalizationDescLieDetector) }
                        Data { description: i18n.t(Key::LocalizationDescLieDetectorTitle) }
                    }
                }
            }
            div { class: "grid grid-cols-2 gap-3 mt-3",
                Button {
                    style: ButtonStyle::Primary,
                    on_click: move |_| async move {
                        save_capture_image(false).await;
                    },

                    {i18n.t(Key::LocalizationCaptureColor)}
                }
                Button {
                    style: ButtonStyle::Primary,
                    on_click: move |_| async move {
                        save_capture_image(true).await;
                    },

                    {i18n.t(Key::LocalizationCaptureGrayscale)}
                }
            }
        }
    }
}

#[component]
fn SectionPopups() -> Element {
    let context = use_context::<LocalizationContext>();
    let localization = context.localization;
    let save_localization = context.save_localization;
    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::LocalizationSectionPopups),
            div { class: "grid grid-cols-2  gap-4",
                LocalizationTemplateInput {
                    label: "Confirm",
                    template: DetectionTemplate::PopupConfirm,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_confirm_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_confirm_base64,
                }
                LocalizationTemplateInput {
                    label: "Yes",
                    template: DetectionTemplate::PopupYes,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_yes_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_yes_base64,
                }
                LocalizationTemplateInput {
                    label: "Next",
                    template: DetectionTemplate::PopupNext,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_next_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_next_base64,
                }
                LocalizationTemplateInput {
                    label: "End chat",
                    template: DetectionTemplate::PopupEndChat,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_end_chat_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_end_chat_base64,
                }
                LocalizationTemplateInput {
                    label: "Ok (new)",
                    template: DetectionTemplate::PopupOkNew,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_ok_new_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_ok_new_base64,
                }
                LocalizationTemplateInput {
                    label: "Ok (old)",
                    template: DetectionTemplate::PopupOkOld,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_ok_old_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_ok_old_base64,
                }
                LocalizationTemplateInput {
                    label: "Cancel (new)",
                    template: DetectionTemplate::PopupCancelNew,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_cancel_new_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_cancel_new_base64,
                }
                LocalizationTemplateInput {
                    label: "Cancel (old)",
                    template: DetectionTemplate::PopupCancelOld,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            popup_cancel_old_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().popup_cancel_old_base64,
                }
            }
        }
    }
}

#[component]
fn SectionHexa() -> Element {
    let context = use_context::<LocalizationContext>();
    let localization = context.localization;
    let save_localization = context.save_localization;
    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::LocalizationSectionHexa),
            div { class: "grid grid-cols-2 gap-4",
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationErdaConversionButton),
                    template: DetectionTemplate::HexaErdaConversionButton,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            hexa_erda_conversion_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().hexa_erda_conversion_button_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationHexaBoosterButton),
                    template: DetectionTemplate::HexaBoosterButton,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            hexa_booster_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().hexa_booster_button_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationMaxButton),
                    template: DetectionTemplate::HexaMaxButton,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            hexa_max_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().hexa_max_button_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationConvertButton),
                    template: DetectionTemplate::HexaConvertButton,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            hexa_convert_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().hexa_convert_button_base64,
                }
            }
        }
    }
}

#[component]
fn SectionFamiliars() -> Element {
    let context = use_context::<LocalizationContext>();
    let localization = context.localization;
    let save_localization = context.save_localization;
    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::LocalizationSectionFamiliars),
            div { class: "grid grid-cols-2 gap-4",
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationLevelSortButton),
                    template: DetectionTemplate::FamiliarsLevelSort,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            familiar_level_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().familiar_level_button_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationSaveButton),
                    template: DetectionTemplate::FamiliarsSaveButton,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            familiar_save_button_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().familiar_save_button_base64,
                }
            }
        }
    }
}

#[component]
fn SectionOthers() -> Element {
    let context = use_context::<LocalizationContext>();
    let localization = context.localization;
    let save_localization = context.save_localization;
    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::LocalizationSectionOthers),
            div { class: "grid grid-cols-2 gap-4",
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationCashShop),
                    template: DetectionTemplate::CashShop,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            cash_shop_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().cash_shop_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationChangeChannel),
                    template: DetectionTemplate::ChangeChannel,
                    tooltip: i18n.t(Key::LocalizationGrayscaleTooltip),
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            change_channel_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().change_channel_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationTimer),
                    template: DetectionTemplate::Timer,
                    tooltip: i18n.t(Key::LocalizationGrayscaleTooltip),
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            timer_base64: to_base64(image, true).await,
                            ..localization()
                        });
                    },
                    value: localization().timer_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationLieDetectorNew),
                    template: DetectionTemplate::LieDetectorNew,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            lie_detector_new_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().lie_detector_new_base64,
                }
                LocalizationTemplateInput {
                    label: i18n.t(Key::LocalizationLieDetectorOld),
                    template: DetectionTemplate::LieDetectorOld,
                    on_value: move |image: Option<Vec<u8>>| async move {
                        save_localization(Localization {
                            lie_detector_old_base64: to_base64(image, false).await,
                            ..localization()
                        });
                    },
                    value: localization().lie_detector_old_base64,
                }
            }
        }
    }
}

#[component]
fn LocalizationTemplateInput(
    label: &'static str,
    template: DetectionTemplate,
    #[props(default)] tooltip: Option<String>,
    on_value: Callback<Option<Vec<u8>>>,
    value: ReadSignal<Option<String>>,
) -> Element {
    let read_file = use_callback(move |file: FileData| async move {
        on_value(file.read_bytes().await.ok().map(Vec::from));
    });
    let mut base64 = use_signal(String::default);
    let i18n = use_i18n();

    use_effect(move || {
        if let Some(value) = value() {
            base64.set(value);
        } else {
            spawn(async move {
                base64.set(query_template(template).await);
            });
        }
    });

    rsx! {
        div { class: "flex gap-2",
            div { class: "flex-grow",
                Labeled { label, tooltip,
                    div { class: "h-6 border-b border-primary-border pb-0.5",
                        img {
                            src: format!("data:image/png;base64,{}", base64()),
                            class: "h-full",
                        }
                    }
                }
            }
            div { class: "flex items-end",
                Button {
                    class: "w-14",
                    style: ButtonStyle::Primary,
                    on_click: move |_| {
                        on_value(None);
                    },

                    {i18n.t(Key::CommonReset)}
                }
            }
            div { class: "flex items-end",
                FileInput {
                    on_file: move |file| async move {
                        read_file(file).await;
                    },
                    accept: ".png,image/png",
                    Button { class: "w-14", style: ButtonStyle::Primary, {i18n.t(Key::CommonReplace)} }
                }
            }
        }
    }
}

async fn to_base64(image: Option<Vec<u8>>, is_grayscale: bool) -> Option<String> {
    OptionFuture::from(image.map(|image| convert_image_to_base64(image, is_grayscale)))
        .await
        .flatten()
}
