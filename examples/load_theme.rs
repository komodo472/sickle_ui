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
        .run();
}

fn setup(mut commands: Commands, theme_data: ResMut<ThemeData>, asset_server: Res<AssetServer>) {
    // Load the theme
    let theme_handle = asset_server.load::<ThemeColors>("themes/material-theme.json");
    commands.insert_resource(ThemeData {
        active_scheme: Scheme::Light(Contrast::Standard),
        theme_handle: Some(theme_handle),
        ..Default::default()
    });

    // The main camera which will render UI
    commands.spawn((Camera2dBundle::default(), IsDefaultUiCamera));

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
                                                        Material3::On(On::Primary),
                                                        Material3::Accent(Accent::Primary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "Secondary".into(),
                                                        "S-40".into(),
                                                        Material3::On(On::Secondary),
                                                        Material3::Accent(Accent::Secondary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "Tertiary".into(),
                                                    "T-40".into(),
                                                    Material3::On(On::Tertiary),
                                                    Material3::Accent(Accent::Tertiary),
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
                                                        Material3::Accent(Accent::Primary),
                                                        Material3::On(On::Primary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "On Secondary".into(),
                                                        "S-100".into(),
                                                        Material3::Accent(Accent::Secondary),
                                                        Material3::On(On::Secondary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "On Tertiary".into(),
                                                    "T-100".into(),
                                                    Material3::Accent(Accent::Tertiary),
                                                    Material3::On(On::Tertiary),
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
                                                        Material3::On(On::PrimaryContainer),
                                                        Material3::Container(Container::Primary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "Secondary Container".into(),
                                                        "S-90".into(),
                                                        Material3::On(On::SecondaryContainer),
                                                        Material3::Container(Container::Secondary),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "Tertiary Container".into(),
                                                    "T-90".into(),
                                                    Material3::On(On::TertiaryContainer),
                                                    Material3::Container(Container::Tertiary),
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
                                                        Material3::Container(Container::Primary),
                                                        Material3::On(On::PrimaryContainer),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column
                                                    .color_block(
                                                        "On Secondary Container".into(),
                                                        "S-10".into(),
                                                        Material3::Container(Container::Secondary),
                                                        Material3::On(On::SecondaryContainer),
                                                    )
                                                    .style()
                                                    .margin(UiRect::right(Val::Px(
                                                        theme_data.spacing.gaps.small,
                                                    )));
                                                column.color_block(
                                                    "On Tertiary Container".into(),
                                                    "T-10".into(),
                                                    Material3::Container(Container::Tertiary),
                                                    Material3::On(On::TertiaryContainer),
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
                                                    Material3::On(On::Surface),
                                                    Material3::Surface(Surface::SurfaceDim),
                                                );
                                                column.color_block(
                                                    "Surface".into(),
                                                    "N-98".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Surface(Surface::Surface),
                                                );
                                                column.color_block(
                                                    "Surface Bright".into(),
                                                    "N-98".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Surface(Surface::SurfaceBright),
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
                                                    Material3::On(On::Surface),
                                                    Material3::Container(Container::SurfaceLowest),
                                                );
                                                column.color_block(
                                                    "Surf. Container Low".into(),
                                                    "N-96".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Container(Container::SurfaceLow),
                                                );
                                                column.color_block(
                                                    "Surf. Container".into(),
                                                    "N-94".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Container(Container::SurfaceMid),
                                                );
                                                column.color_block(
                                                    "Surf. Container High".into(),
                                                    "N-92".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Container(Container::SurfaceHigh),
                                                );
                                                column.color_block(
                                                    "Surf. Container Highest".into(),
                                                    "N-90".into(),
                                                    Material3::On(On::Surface),
                                                    Material3::Container(Container::SurfaceHighest),
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
                                                    Material3::Surface(Surface::Surface),
                                                    Material3::On(On::Surface),
                                                );
                                                column.color_block(
                                                    "On Surface Var.".into(),
                                                    "NV-30".into(),
                                                    Material3::Surface(Surface::SurfaceVariant),
                                                    Material3::On(On::SurfaceVariant),
                                                );
                                                column.color_block(
                                                    "Outline".into(),
                                                    "NV-50".into(),
                                                    Material3::Surface(Surface::Surface),
                                                    Material3::Accent(Accent::Outline),
                                                );
                                                column.color_block(
                                                    "Outline Variant".into(),
                                                    "NV-80".into(),
                                                    Material3::Surface(Surface::InverseSurface),
                                                    Material3::Accent(Accent::OutlineVariant),
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
                                                    Material3::On(On::Error),
                                                    Material3::Accent(Accent::Error),
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
                                                    Material3::Accent(Accent::Error),
                                                    Material3::On(On::Error),
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
                                                    Material3::On(On::ErrorContainer),
                                                    Material3::Container(Container::Error),
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
                                                    Material3::Container(Container::Error),
                                                    Material3::On(On::ErrorContainer),
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
                                                    Material3::On(On::InverseSurface),
                                                    Material3::Surface(Surface::InverseSurface),
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
                                                    Material3::Surface(Surface::InverseSurface),
                                                    Material3::On(On::InverseSurface),
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
                                                    Material3::On(On::Surface),
                                                    Material3::Accent(Accent::InversePrimary),
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
                                                    Material3::On(On::Primary),
                                                    Material3::Accent(Accent::Scrim),
                                                )
                                                .style()
                                                .margin(UiRect::right(Val::Px(
                                                    theme_data.spacing.gaps.small,
                                                )));
                                                row.color_block(
                                                    "Shadow".into(),
                                                    "N-0".into(),
                                                    Material3::On(On::Primary),
                                                    Material3::Accent(Accent::Shadow),
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
        match block.material_text_color {
            Material3::Surface(surface) => {
                style_builder.font_color(theme_data.colors().surface(surface))
            }
            Material3::Accent(accent) => {
                style_builder.font_color(theme_data.colors().accent(accent))
            }
            Material3::Container(container) => {
                style_builder.font_color(theme_data.colors().container(container))
            }
            Material3::On(on) => style_builder.font_color(theme_data.colors().on(on)),
        };

        style_builder
            .switch_target(ColorBlock::MATERIAL_P_TEXT)
            .sized_font(font.clone());
        match block.material_text_color {
            Material3::Surface(surface) => {
                style_builder.font_color(theme_data.colors().surface(surface))
            }
            Material3::Accent(accent) => {
                style_builder.font_color(theme_data.colors().accent(accent))
            }
            Material3::Container(container) => {
                style_builder.font_color(theme_data.colors().container(container))
            }
            Material3::On(on) => style_builder.font_color(theme_data.colors().on(on)),
        };
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
        text_color: Material3,
        background_color: Material3,
    ) -> UiBuilder<Entity>;
}

impl UiHeadlineExt for UiBuilder<'_, Entity> {
    fn color_block(
        &mut self,
        name: String,
        p: String,
        text_color: Material3,
        background_color: Material3,
    ) -> UiBuilder<Entity> {
        let mut color_block = ColorBlock::default();
        let mut frame = self.container(ColorBlock::frame(name.clone()), |container| {
            color_block.material_name_text = container
                .spawn(TextBundle {
                    text: Text::from_section(name, TextStyle::default())
                        .with_justify(JustifyText::Left),
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
                    text: Text::from_section(p, TextStyle::default())
                        .with_justify(JustifyText::Right),
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
        color_block.material_text_color = text_color;
        color_block.material_background_color = background_color;

        frame.insert(color_block);

        let id = frame.id();

        self.commands().ui_builder(id)
    }
}

#[derive(Debug, Reflect)]
pub enum Material3 {
    Surface(Surface),
    Accent(Accent),
    Container(Container),
    On(On),
}
