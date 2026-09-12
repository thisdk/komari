use backend::{
    ActionConfiguration, ActionConfigurationCondition, ActionKeyWith, KeyBinding, LinkKeyBinding,
    WaitAfterBuffered,
};
use dioxus::prelude::*;

use crate::{
    characters::{
        CharactersCheckbox, CharactersKeyInput, CharactersMillisInput, CharactersNumberU32Input,
        CharactersSelect,
    },
    components::{
        ContentAlign, ContentSide,
        button::{Button, ButtonStyle},
        checkbox::Checkbox,
        icons::XIcon,
        list::{List, ListItem, MoveEvent},
        popup::{self, PopupContext, PopupTrigger},
    },
    i18n::{Key, use_i18n},
};

#[derive(Debug)]
pub struct ItemToggleEvent {
    pub enabled: bool,
    pub index: usize,
}

#[derive(Debug)]
pub struct ItemMoveEvent {
    pub from_index: usize,
    pub to_index: usize,
}

#[derive(Debug)]
pub struct ItemClickEvent {
    pub action: ActionConfiguration,
    pub index: usize,
}

#[derive(PartialEq, Clone)]
enum PopupContent {
    None,
    Add(ActionConfiguration),
    Edit {
        action: ActionConfiguration,
        index: usize,
    },
}

#[component]
pub fn ActionConfigurationsList(
    disabled: bool,
    on_item_add: Callback<ActionConfiguration>,
    on_item_click: Callback<ItemClickEvent>,
    on_item_delete: Callback<usize>,
    on_item_toggle: Callback<ItemToggleEvent>,
    on_item_move: Callback<ItemMoveEvent>,
    actions: Vec<ActionConfiguration>,
) -> Element {
    let mut popup_content = use_signal(|| PopupContent::None);
    let mut popup_open = use_signal(|| false);
    let i18n = use_i18n();

    rsx! {
        PopupContext {
            open: popup_open,
            on_open: move |open| {
                popup_open.set(open);
            },

            List {
                class: "flex flex-col",
                on_move: move |event: MoveEvent| {
                    on_item_move(ItemMoveEvent {
                        from_index: event.from,
                        to_index: event.to,
                    });
                },
                for (index , action) in actions.into_iter().enumerate() {
                    ListItem { class: "flex items-end",
                        div {
                            class: "flex group flex-grow",
                            onclick: move |_| {
                                popup_content
                                    .set(PopupContent::Edit {
                                        action,
                                        index,
                                    });
                            },

                            PopupTrigger { class: "flex-grow",
                                Item { action }
                            }

                            Icons {
                                condition: action.condition,
                                on_item_delete: move |_| {
                                    on_item_delete(index);
                                },
                            }
                        }

                        div { class: "w-8 flex flex-col items-end",
                            if !matches!(action.condition, ActionConfigurationCondition::Linked) {
                                Checkbox {
                                    on_checked: move |enabled| {
                                        on_item_toggle(ItemToggleEvent { enabled, index });
                                    },
                                    checked: action.enabled,
                                }
                            }
                        }
                    }
                }
            }

            PopupTrigger {
                Button {
                    style: ButtonStyle::Secondary,
                    class: "w-full mt-2",
                    on_click: move |_| {
                        popup_content.set(PopupContent::Add(ActionConfiguration::default()));
                    },
                    disabled,

                    {i18n.t(Key::CommonAddAction)}
                }
            }

            PopupActionConfigurationContent {
                modifying: matches!(popup_content(), PopupContent::Edit { .. }),
                can_create_linked_action: match popup_content() {
                    PopupContent::None | PopupContent::Add(_) => false,
                    PopupContent::Edit { index, .. } => index != 0,
                },
                on_copy: move |_| {
                    let content = popup_content.peek().clone();
                    match content {
                        PopupContent::Add(_) | PopupContent::None => unreachable!(),
                        PopupContent::Edit { action, .. } => {
                            popup_content.set(PopupContent::Add(action));
                        }
                    }
                },
                on_cancel: move |_| {
                    popup_open.set(false);
                },
                on_value: move |value| {
                    match popup_content.peek().clone() {
                        PopupContent::None => unreachable!(),
                        PopupContent::Add(_) => {
                            on_item_add(value);
                        }
                        PopupContent::Edit { index, .. } => {
                            on_item_click(ItemClickEvent {
                                action: value,
                                index,
                            });
                        }
                    }
                    popup_open.set(false);
                },
                value: match popup_content() {
                    PopupContent::None => None,
                    PopupContent::Add(action) | PopupContent::Edit { action, .. } => Some(action),
                },
            }
        }
    }
}

#[component]
fn Icons(condition: ActionConfigurationCondition, on_item_delete: Callback) -> Element {
    let container_margin = if matches!(condition, ActionConfigurationCondition::Linked) {
        ""
    } else {
        "mt-2"
    };

    rsx! {
        div { class: "self-stretch invisible group-hover:visible group-hover:bg-secondary-surface flex items-center {container_margin} pr-1",
            div {
                class: "size-fit",
                onclick: move |e| {
                    e.stop_propagation();
                    on_item_delete(());
                },
                XIcon { class: "size-3" }
            }
        }
    }
}

#[component]
fn Item(action: ActionConfiguration) -> Element {
    const ITEM_TEXT_CLASS: &str =
        "text-center inline-block pt-1 text-ellipsis overflow-hidden whitespace-nowrap";
    const ITEM_BORDER_CLASS: &str = "border-r-2 border-secondary-border";

    let i18n = use_i18n();
    let ActionConfiguration {
        key,
        key_hold_millis,
        key_hold_buffered_to_wait_after,
        link_key,
        count,
        condition,
        with,
        wait_before_millis,
        wait_after_millis,
        wait_after_buffered,
        ..
    } = action;

    let linked_action = if matches!(condition, ActionConfigurationCondition::Linked) {
        ""
    } else {
        "mt-2"
    };

    let key_hold_buffered = if key_hold_buffered_to_wait_after {
        "⁺".to_string()
    } else {
        "".to_string()
    };
    let key_hold = if key_hold_millis > 0 {
        " ⤓".to_string()
    } else {
        "".to_string()
    };
    let key = format!("{key}{key_hold}{key_hold_buffered}");
    let link_key = match link_key {
        LinkKeyBinding::Before(key) => format!("{key} ↝ "),
        LinkKeyBinding::After(key) => format!("{key} ↜ "),
        LinkKeyBinding::AtTheSame(key) => format!("{key} ↭ "),
        LinkKeyBinding::Along(key) => format!("{key} ↷ "),
        LinkKeyBinding::None => "".to_string(),
    };

    let millis = if let ActionConfigurationCondition::EveryMillis(millis) = condition {
        format!("⟳ {:.2}s / ", millis as f32 / 1000.0)
    } else {
        "".to_string()
    };

    let wait_before_secs = if wait_before_millis > 0 {
        Some(format!("⏱︎ {:.2}s", wait_before_millis as f32 / 1000.0))
    } else {
        None
    };

    let wait_after_buffered = if !matches!(wait_after_buffered, WaitAfterBuffered::None) {
        "⁺".to_string()
    } else {
        "".to_string()
    };
    let wait_after_secs = if wait_after_millis > 0 {
        Some(format!(
            "{:.2}s{wait_after_buffered}",
            wait_after_millis as f32 / 1000.0
        ))
    } else {
        None
    };

    let wait_secs = match (wait_before_secs, wait_after_secs) {
        (Some(before), None) => format!("{before} - 0.00s / "),
        (None, None) => "".to_string(),
        (None, Some(after)) => format!("⏱︎ 0.00s - {after} / "),
        (Some(before), Some(after)) => format!("{before} - {after} / "),
    };

    let with = i18n.t(match with {
        ActionKeyWith::Any => Key::CommonAny,
        ActionKeyWith::Stationary => Key::CommonStationary,
        ActionKeyWith::DoubleJump => Key::CommonDoubleJump,
    });

    rsx! {
        div { class: "grid grid-cols-[100px_auto] h-6 text-xs text-secondary-text group-hover:bg-secondary-surface {linked_action}",
            div { class: "{ITEM_BORDER_CLASS} {ITEM_TEXT_CLASS}", "{link_key}{key} × {count}" }
            div { class: "pl-1 pr-13 {ITEM_TEXT_CLASS}", "{millis}{wait_secs}{with}" }
        }
    }
}

#[component]
fn PopupActionConfigurationContent(
    modifying: bool,
    can_create_linked_action: bool,
    on_copy: Callback,
    on_cancel: Callback,
    on_value: Callback<ActionConfiguration>,
    value: Option<ActionConfiguration>,
) -> Element {
    let i18n = use_i18n();
    let section_text = if modifying {
        i18n.t(Key::CharactersModifyFixedAction).to_string()
    } else {
        i18n.t(Key::CharactersAddNewFixedAction).to_string()
    };

    rsx! {
        popup::PopupContent { title: section_text,
            ActionConfigurationInput {
                modifying,
                can_create_linked_action,
                on_copy,
                on_cancel,
                on_value,
                value: value.unwrap_or_default(),
            }
        }
    }
}

#[component]
fn ActionConfigurationInput(
    modifying: bool,
    can_create_linked_action: bool,
    on_copy: Callback,
    on_cancel: Callback,
    on_value: Callback<ActionConfiguration>,
    value: ReadSignal<ActionConfiguration>,
) -> Element {
    let mut action = use_signal(&*value);
    let millis = use_memo(move || match action().condition {
        ActionConfigurationCondition::EveryMillis(millis) => Some(millis),
        ActionConfigurationCondition::Linked => None,
    });
    let i18n = use_i18n();

    use_effect(move || {
        action.set(value());
    });

    rsx! {
        div { class: "grid grid-cols-3 gap-3 pb-10 overflow-y-auto",
            if modifying {
                div { class: "flex flex-col col-span-3",
                    Button {
                        style: ButtonStyle::Primary,
                        on_click: on_copy,
                        class: "col-span-3",
                        {i18n.t(Key::CommonCopy)}
                    }
                    div { class: "border-b border-primary-border" }
                }
            }
            // Key, count and link key
            CharactersKeyInput {
                label: i18n.t(Key::CommonKey),
                input_class: "border border-primary-border",
                on_value: move |key: Option<KeyBinding>| {
                    let mut action = action.write();
                    action.key = key.expect("not optional");
                },
                value: Some(action().key),
            }
            div { class: "grid grid-cols-2 gap-3",
                CharactersNumberU32Input {
                    label: i18n.t(Key::ActionUseCount),
                    on_value: move |count| {
                        let mut action = action.write();
                        action.count = count;
                    },
                    value: action().count,
                }
                CharactersMillisInput {
                    label: i18n.t(Key::ActionHoldFor),
                    on_value: move |millis| {
                        let mut action = action.write();
                        action.key_hold_millis = millis;
                    },
                    value: action().key_hold_millis,
                }
            }
            CharactersCheckbox {
                label: i18n.t(Key::ActionHoldingBuffered),
                tooltip: i18n.t(Key::ActionHoldingBufferedTooltip),
                tooltip_align: ContentAlign::End,
                tooltip_side: ContentSide::Bottom,
                on_checked: move |checked| {
                    let mut action = action.write();
                    action.key_hold_buffered_to_wait_after = checked;
                },
                checked: action().key_hold_buffered_to_wait_after,
            }

            CharactersKeyInput {
                label: i18n.t(Key::ActionLinkKey),
                input_class: "border border-primary-border",
                disabled: matches!(action().link_key, LinkKeyBinding::None),
                on_value: move |key: Option<KeyBinding>| {
                    let mut action = action.write();
                    action.link_key = action.link_key.with_key(key.expect("not optional"));
                },
                value: action().link_key.key().unwrap_or_default(),
            }
            CharactersSelect::<LinkKeyBinding> {
                label: i18n.t(Key::ActionLinkKeyType),
                on_selected: move |link_key: LinkKeyBinding| {
                    let mut action = action.write();
                    action.link_key = link_key;
                },
                selected: action().link_key,
            }
            if can_create_linked_action {
                CharactersCheckbox {
                    label: i18n.t(Key::ActionLinkedAction),
                    checked: matches!(action().condition, ActionConfigurationCondition::Linked),
                    on_checked: move |is_linked: bool| {
                        let mut action = action.write();
                        action.condition = if is_linked {
                            ActionConfigurationCondition::Linked
                        } else {
                            value.peek().condition
                        };
                    },
                }
            } else {
                div {} // Spacer
            }

            // Use with
            CharactersSelect::<ActionKeyWith> {
                label: i18n.t(Key::ActionUseWith),
                on_selected: move |with| {
                    let mut action = action.write();
                    action.with = with;
                },
                selected: action().with,
            }
            CharactersMillisInput {
                label: i18n.t(Key::ActionUseEvery),
                disabled: millis().is_none(),
                on_value: move |new_millis| {
                    if millis.peek().is_some() {
                        let mut action = action.write();
                        action.condition = ActionConfigurationCondition::EveryMillis(new_millis);
                    }
                },
                value: millis().unwrap_or_default(),
            }
            div {} // Spacer

            // Wait before use
            CharactersMillisInput {
                label: i18n.t(Key::ActionWaitBeforeUse),
                on_value: move |millis| {
                    let mut action = action.write();
                    action.wait_before_millis = millis;
                },
                value: action().wait_before_millis,
            }
            CharactersMillisInput {
                label: i18n.t(Key::ActionWaitRandomRange),
                on_value: move |millis| {
                    let mut action = action.write();
                    action.wait_before_millis_random_range = millis;
                },
                value: action().wait_before_millis_random_range,
            }
            div {} // Spacer

            // Wait after use
            CharactersMillisInput {
                label: i18n.t(Key::ActionWaitAfterUse),
                on_value: move |millis| {
                    let mut action = action.write();
                    action.wait_after_millis = millis;
                },
                value: action().wait_after_millis,
            }
            CharactersMillisInput {
                label: i18n.t(Key::ActionWaitRandomRange),
                on_value: move |millis| {
                    let mut action = action.write();
                    action.wait_after_millis_random_range = millis;
                },
                value: action().wait_after_millis_random_range,
            }
            CharactersSelect::<WaitAfterBuffered> {
                label: i18n.t(Key::ActionWaitAfterBuffered),
                tooltip: i18n.t(Key::ActionWaitAfterBufferedTooltip),
                tooltip_align: ContentAlign::End,
                on_selected: move |wait_after_buffered: WaitAfterBuffered| {
                    let mut action = action.write();
                    action.wait_after_buffered = wait_after_buffered;
                },
                selected: action().wait_after_buffered,
            }
        }
        div { class: "flex w-full gap-3 absolute bottom-0 py-2 bg-secondary-surface",
            Button {
                class: "flex-grow",
                style: ButtonStyle::OutlinePrimary,
                on_click: move |_| {
                    on_value(*action.peek());
                },
                if modifying {
                    {i18n.t(Key::CommonSave)}
                } else {
                    {i18n.t(Key::CommonAdd)}
                }
            }
            Button {
                class: "flex-grow",
                style: ButtonStyle::OutlineSecondary,
                on_click: move |_| {
                    on_cancel(());
                },
                {i18n.t(Key::CommonCancel)}
            }
        }
    }
}
