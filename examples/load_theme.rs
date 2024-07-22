//! An example loading the material 3 theme from an asset. Replica of https://material-foundation.github.io/material-theme-builder/
use bevy::prelude::*;

use sickle_ui::{
    prelude::*,
    theme::theme_colors::{ThemeColors, ThemeColorsLoader},
    SickleUiPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sickle UI -  Material 3 Theme".into(),
                resolution: (1280., 720.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(SickleUiPlugin)
        .add_plugins(ComponentThemePlugin::<ColorBlock>::default())
        .init_asset::<ThemeColors>()
        .init_asset_loader::<ThemeColorsLoader>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            check_themes_changed.run_if(resource_exists::<MyTheme>),
        )
        .run();
}

#[derive(Resource, Default)]
struct MyTheme(Handle<ThemeColors>);

fn check_themes_changed(
    mut commands: Commands,
    mut theme_data: ResMut<ThemeData>,
    my_theme: Res<MyTheme>,
    themes: Res<Assets<ThemeColors>>,
) {
    let theme_handle = my_theme.0.clone();
    if themes.is_changed() && themes.get(&theme_handle).is_some() {
        theme_data.colors = themes.get(&theme_handle).unwrap().clone();
        commands.insert_resource(theme_data.clone());
        builder(commands, theme_data);
    }
}

fn setup(mut commands: Commands, theme_data: ResMut<ThemeData>, asset_server: Res<AssetServer>) {
    // Load the theme
    let theme: Handle<ThemeColors> = asset_server.load("themes/material-theme.json");
    commands.insert_resource(MyTheme(theme));

    // The main camera which will render UI
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));
}

fn builder(mut commands: Commands, theme_data: ResMut<ThemeData>) {
    commands
        .ui_builder(UiRoot)
        .row(|row| {
            row.column(|column| {
                column
                    .row(|row| {
                        row.column(|column| {
                            column
                                .row(|row| {
                                    let headline_font = theme_data.text.get(
                                        FontStyle::Headline,
                                        FontScale::Large,
                                        FontType::Regular,
                                    );
                                    row.spawn(TextBundle {
                                        text: Text {
                                            sections: vec![TextSection::new(
                                                "Light Scheme",
                                                TextStyle {
                                                    color: bevy::color::palettes::css::BLACK.into(),
                                                    ..Default::default()
                                                },
                                            )],
                                            justify: JustifyText::Center,
                                            ..default()
                                        },
                                        ..default()
                                    })
                                    .style()
                                    .sized_font(headline_font);
                                })
                                .style()
                                .width(Val::Percent(100.0))
                                .padding(UiRect::bottom(Val::Px(theme_data.spacing.gaps.large)));
                            column
                                .row(|row| {
                                    row.column(|column| {
                                        column
                                            .row(|column| {
                                                column
                                                    .color_block(
                                                        "Primary".into(),
                                                        "P-40".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_100
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_40
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "Secondary".into(),
                                                        "S-40".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_100
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_40
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "Tertiary".into(),
                                                    "T-40".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .tertiary
                                                        .p_100
                                                        .into(),
                                                    theme_data.colors.palettes.tertiary.p_40.into(),
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|column| {
                                                column
                                                    .color_block(
                                                        "On Primary".into(),
                                                        "P-100".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_40
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_100
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "On Secondary".into(),
                                                        "S-100".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_40
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_100
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "On Tertiary".into(),
                                                    "T-100".into(),
                                                    theme_data.colors.palettes.tertiary.p_40.into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .tertiary
                                                        .p_100
                                                        .into(),
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|column| {
                                                column
                                                    .color_block(
                                                        "Primary Container".into(),
                                                        "P-90".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_10
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_90
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "Secondary Container".into(),
                                                        "S-90".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_10
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_90
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "Tertiary Container".into(),
                                                    "T-90".into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                    theme_data.colors.palettes.tertiary.p_90.into(),
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|column| {
                                                column
                                                    .color_block(
                                                        "On Primary Container".into(),
                                                        "P-10".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_90
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .primary
                                                            .p_10
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "On Secondary Container".into(),
                                                        "S-10".into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_90
                                                            .into(),
                                                        theme_data
                                                            .colors
                                                            .palettes
                                                            .secondary
                                                            .p_10
                                                            .into(),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "On Tertiary Container".into(),
                                                    "T-10".into(),
                                                    theme_data.colors.palettes.tertiary.p_90.into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.large,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|column| {
                                                column.color_block(
                                                    "Suface Dim".into(),
                                                    "N-87".into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .on_surface,
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_dim,
                                                );
                                                column.color_block(
                                                    "Surface".into(),
                                                    "N-98".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .secondary
                                                        .p_10
                                                        .into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .secondary
                                                        .p_90
                                                        .into(),
                                                );
                                                column.color_block(
                                                    "Surface Bright".into(),
                                                    "N-98".into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                    theme_data.colors.palettes.tertiary.p_90.into(),
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|column| {
                                                column.color_block(
                                                    "Suf. Container Lowest".into(),
                                                    "N-100".into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .on_surface,
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_container_lowest,
                                                );
                                                column.color_block(
                                                    "Surf. Container Low".into(),
                                                    "N-96".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .secondary
                                                        .p_10
                                                        .into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_container_low,
                                                );
                                                column.color_block(
                                                    "Surf. Container".into(),
                                                    "N-94".into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_container,
                                                );
                                                column.color_block(
                                                    "Surf. Container High".into(),
                                                    "N-92".into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_container_high,
                                                );
                                                column.color_block(
                                                    "Surf. Container Highest".into(),
                                                    "N-90".into(),
                                                    theme_data.colors.palettes.tertiary.p_10.into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .surface_container_highest,
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|column| {
                                                column.color_block(
                                                    "On Suface".into(),
                                                    "N-10".into(),
                                                    theme_data.colors.palettes.neutral.p_100.into(),
                                                    theme_data.colors.palettes.neutral.p_10.into(),
                                                );
                                                column.color_block(
                                                    "On Surface Var.".into(),
                                                    "NV-30".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_100
                                                        .into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_30
                                                        .into(),
                                                );
                                                column.color_block(
                                                    "Outline".into(),
                                                    "NV-50".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_100
                                                        .into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_50
                                                        .into(),
                                                );
                                                column.color_block(
                                                    "Outline Variant".into(),
                                                    "NV-80".into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_100
                                                        .into(),
                                                    theme_data
                                                        .colors
                                                        .palettes
                                                        .neutral_variant
                                                        .p_10
                                                        .into(),
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                    })
                                    .style()
                                    .width(Val::Percent(75.0))
                                    .height(Val::Percent(100.0));
                                    row.column(|column| {
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Error".into(),
                                                    "E-40".into(),
                                                    theme_data.colors.schemes.light.colors.on_error,
                                                    theme_data.colors.schemes.light.colors.error,
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "On Error".into(),
                                                    "E-100".into(),
                                                    theme_data.colors.schemes.light.colors.error,
                                                    theme_data.colors.schemes.light.colors.on_error,
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Error Container".into(),
                                                    "E-90".into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .on_error_container,
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .error_container,
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "On Error Container".into(),
                                                    "E-10".into(),
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .error_container,
                                                    theme_data
                                                        .colors
                                                        .schemes
                                                        .light
                                                        .colors
                                                        .on_error_container,
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.large,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Inverse Surface".into(),
                                                    "N-90".into(),
                                                    theme_data.colors.palettes.neutral.p_10.into(),
                                                    theme_data.colors.palettes.neutral.p_90.into(),
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(20.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Inverse On Surface".into(),
                                                    "N-20".into(),
                                                    theme_data.colors.palettes.neutral.p_100.into(),
                                                    theme_data.colors.palettes.neutral.p_20.into(),
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Inverse Primary".into(),
                                                    "P-40".into(),
                                                    theme_data.colors.palettes.primary.p_100.into(),
                                                    theme_data.colors.palettes.primary.p_40.into(),
                                                );
                                            })
                                            .style()
                                            .padding(UiRect::bottom(Val::Px(
                                                theme_data.spacing.gaps.small,
                                            )))
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                        column
                                            .row(|row| {
                                                row.color_block(
                                                    "Scrim".into(),
                                                    "N-0".into(),
                                                    theme_data.colors.palettes.neutral.p_100.into(),
                                                    theme_data.colors.palettes.neutral.p_0.into(),
                                                )
                                                .style()
                                                .margin(UiRect::right(Val::Px(
                                                    theme_data.spacing.gaps.small,
                                                )));
                                                row.color_block(
                                                    "Shadow".into(),
                                                    "N-0".into(),
                                                    theme_data.colors.palettes.neutral.p_100.into(),
                                                    theme_data.colors.palettes.neutral.p_0.into(),
                                                );
                                            })
                                            .style()
                                            .width(Val::Percent(100.0))
                                            .height(Val::Percent(10.0));
                                    })
                                    .style()
                                    .padding(UiRect::left(Val::Px(theme_data.spacing.gaps.large)))
                                    .width(Val::Percent(25.0))
                                    .height(Val::Percent(100.0));
                                })
                                .style()
                                .width(Val::Percent(100.0))
                                .height(Val::Percent(100.0));
                        })
                        .style()
                        .width(Val::Percent(100.0))
                        .padding(UiRect::all(Val::Px(theme_data.spacing.gaps.large)));
                    })
                    .style()
                    .width(Val::Percent(100.0))
                    .height(Val::Percent(100.0));
            })
            .style()
            .width(Val::Percent(95.0))
            .height(Val::Percent(95.0))
            .align_self(AlignSelf::Center)
            .align_items(AlignItems::FlexStart)
            .background_color(bevy::color::palettes::css::GHOST_WHITE.into())
            .border_radius(BorderRadius::all(Val::Px(theme_data.spacing.corners.large)));
        })
        .style()
        .width(Val::Percent(100.0))
        .height(Val::Percent(100.0))
        .justify_content(JustifyContent::Center);
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct ColorBlock {
    material_name_text: Entity,
    material_p_text: Entity,
    material_text_color: Material3,
    material_background_color: Material3,
}

impl Default for ColorBlock {
    fn default() -> Self {
        Self {
            material_name_text: Entity::PLACEHOLDER,
            material_p_text: Entity::PLACEHOLDER,
            material_text_color: Material3::On(On::Primary),
            material_background_color: Material3::Surface(Surface::Background),
        }
    }
}

impl UiContext for ColorBlock {
    fn get(&self, target: &str) -> Result<Entity, String> {
        match target {
            ColorBlock::MATERIAL_NAME_TEXT => Ok(self.material_name_text),
            ColorBlock::MATERIAL_P_TEXT => Ok(self.material_p_text),
            _ => Err(format!(
                "{} doesn't exists for ColorBlock. Possible contexts: {:?}",
                target,
                self.contexts()
            )),
        }
    }

    fn contexts(&self) -> Vec<&'static str> {
        vec![ColorBlock::MATERIAL_NAME_TEXT, ColorBlock::MATERIAL_P_TEXT]
    }
}

impl DefaultTheme for ColorBlock {
    fn default_theme() -> Option<Theme<ColorBlock>> {
        ColorBlock::theme().into()
    }
}

impl ColorBlock {
    pub const MATERIAL_NAME_TEXT: &'static str = "MaterialNameText";
    pub const MATERIAL_P_TEXT: &'static str = "MaterialPText";

    pub fn theme() -> Theme<ColorBlock> {
        let base_theme = PseudoTheme::deferred_context(None, ColorBlock::primary_style);
        Theme::new(vec![base_theme])
    }

    fn primary_style(style_builder: &mut StyleBuilder, block: &ColorBlock, theme_data: &ThemeData) {
        style_builder.padding(UiRect::all(Val::Px(theme_data.spacing.gaps.medium)));

        let font = theme_data
            .text
            .get(FontStyle::Body, FontScale::Large, FontType::Regular);

        match block.material_background_color {
            Material3::Surface(surface) => {
                style_builder.background_color(theme_data.colors().surface(surface))
            }
            Material3::Accent(accent) => {
                style_builder.background_color(theme_data.colors().accent(accent))
            }
            Material3::Container(container) => {
                style_builder.background_color(theme_data.colors().container(container))
            }
            Material3::On(on) => style_builder.background_color(theme_data.colors().on(on)),
        };

        style_builder
            .switch_target(ColorBlock::MATERIAL_NAME_TEXT)
            .sized_font(font.clone());
        style_builder
            .switch_target(ColorBlock::MATERIAL_P_TEXT)
            .sized_font(font.clone());
    }

    pub fn frame(name: String) -> impl Bundle {
        (
            Name::new(format!("ColorBlock [{}]", name)),
            NodeBundle {
                style: Style {
                    height: Val::Percent(100.0),
                    width: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::SpaceBetween,
                    ..Default::default()
                },
                ..default()
            },
        )
    }
}

pub trait UiHeadlineExt {
    fn color_block(
        &mut self,
        name: String,
        name_value: String,
        text_color: Color,
        background_color: Material3,
    ) -> UiBuilder<Entity>;
}

impl UiHeadlineExt for UiBuilder<'_, Entity> {
    fn color_block(
        &mut self,
        name: String,
        p: String,
        text_color: Color,
        background_color: Material3,
    ) -> UiBuilder<Entity> {
        let mut color_block = ColorBlock::default();
        let mut frame = self.container(ColorBlock::frame(name.clone()), |container| {
            color_block.material_name_text = container
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            name,
                            TextStyle {
                                color: text_color,
                                ..Default::default()
                            },
                        )],
                        justify: JustifyText::Left,
                        ..default()
                    },
                    style: Style {
                        justify_self: JustifySelf::Start,
                        align_self: AlignSelf::Start,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .style()
                .id();
            color_block.material_p_text = container
                .spawn(TextBundle {
                    text: Text {
                        sections: vec![TextSection::new(
                            p,
                            TextStyle {
                                color: text_color,
                                ..Default::default()
                            },
                        )],
                        ..default()
                    },
                    style: Style {
                        justify_self: JustifySelf::End,
                        align_self: AlignSelf::End,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .style()
                .id();
        });
        color_block.material_background_color = background_color;
        // frame.style().background_color(background_color);

        frame.insert(color_block);

        let id = frame.id();

        self.commands().ui_builder(id)
    }
}

#[derive(Debug, Reflect)]
enum Material3 {
    Surface(Surface),
    Accent(Accent),
    Container(Container),
    On(On),
}
