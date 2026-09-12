use backend::{Character, KeyBindingConfiguration};
use dioxus::prelude::*;

use crate::{
    characters::{CharactersContext, CharactersKeyBindingConfigurationInput},
    components::section::Section,
    i18n::{Key, use_i18n},
};

#[component]
pub fn SectionKeyBindings() -> Element {
    let context = use_context::<CharactersContext>();
    let character = context.character;
    let save_character = context.save_character;
    let i18n = use_i18n();

    rsx! {
        Section { title: i18n.t(Key::SectionKeyBindings),
            div { class: "grid grid-cols-2 2xl:grid-cols-4 gap-4",
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsRopeLift),
                    optional: true,
                    disabled: character().id.is_none(),
                    on_value: move |ropelift_key| {
                        save_character(Character {
                            ropelift_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().ropelift_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsTeleport),
                    optional: true,
                    disabled: character().id.is_none(),
                    on_value: move |teleport_key| {
                        save_character(Character {
                            teleport_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().teleport_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsJump),
                    disabled: character().id.is_none(),
                    on_value: move |key_config: Option<KeyBindingConfiguration>| {
                        save_character(Character {
                            jump_key: key_config.expect("not optional"),
                            ..character.peek().clone()
                        });
                    },
                    value: character().jump_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsUpJump),
                    optional: true,
                    tooltip: i18n.t(Key::BindingsUpJumpTooltip),
                    disabled: character().id.is_none(),
                    on_value: move |up_jump_key| {
                        save_character(Character {
                            up_jump_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().up_jump_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsInteract),
                    disabled: character().id.is_none(),
                    on_value: move |key_config: Option<KeyBindingConfiguration>| {
                        save_character(Character {
                            interact_key: key_config.expect("not optional"),
                            ..character.peek().clone()
                        });
                    },
                    value: character().interact_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsCashShop),
                    optional: true,
                    disabled: character().id.is_none(),
                    tooltip: i18n.t(Key::BindingsCashShopTooltip),
                    on_value: move |cash_shop_key| {
                        save_character(Character {
                            cash_shop_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().cash_shop_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsToTown),
                    optional: true,
                    disabled: character().id.is_none(),
                    tooltip: i18n.t(Key::BindingsToTownTooltip),
                    on_value: move |to_town_key| {
                        save_character(Character {
                            to_town_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().to_town_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsChangeChannel),
                    optional: true,
                    disabled: character().id.is_none(),
                    tooltip: i18n.t(Key::BindingsChangeChannelTooltip),
                    on_value: move |change_channel_key| {
                        save_character(Character {
                            change_channel_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().change_channel_key,
                }
                CharactersKeyBindingConfigurationInput {
                    label: i18n.t(Key::BindingsFamiliarMenu),
                    optional: true,
                    tooltip: i18n.t(Key::BindingsFamiliarMenuTooltip),
                    disabled: character().id.is_none(),
                    on_value: move |familiar_menu_key| {
                        save_character(Character {
                            familiar_menu_key,
                            ..character.peek().clone()
                        });
                    },
                    value: character().familiar_menu_key,
                }
            }
        }
    }
}
