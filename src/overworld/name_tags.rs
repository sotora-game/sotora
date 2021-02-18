use crate::overworld::CameraObject;
use crate::overworld::StateCleanup;
use crate::UiAssets;
use bevy::prelude::*;

pub struct NameTag {
    /// Text for the name tag
    name: String,
    /// How high above the entity it should sit
    margin: f32,
}
impl NameTag {
    pub fn new(name: String) -> Self {
        Self { name, margin: 1. }
    }
    pub fn new_with_margin(name: String, margin: f32) -> Self {
        Self { name, margin }
    }
}

pub struct NameTagSprite(Entity);

pub fn spawn_name_tag_sprite(
    commands: &mut Commands,
    assets: Res<UiAssets>,
    fonts: Res<Assets<Font>>,
    mut textures: ResMut<Assets<Texture>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(Entity, &NameTag), Added<NameTag>>,
) {
    if let Some(font) = fonts.get(assets.font_regular.clone()) {
        for (entity, name_tag) in query.iter() {
            let text = font.render_text(&name_tag.name, Color::WHITE, 30., 100, 100);
            let text_handle = textures.add(text);

            commands
                .spawn(SpriteBundle {
                    material: color_materials.add(text_handle.into()),

                    sprite: Sprite {
                        size: Vec2::new(1.0, 1.0),
                        ..Default::default()
                    },
                    // We don't bother setting the correct position, as it'll be set by the other system
                    transform: Transform::from_scale(Vec3::new(-0.03, 0.03, 0.03)),
                    ..Default::default()
                })
                .with(StateCleanup)
                .with(NameTagSprite(entity));
        }
    }
}

pub fn move_name_tag_and_rotate(
    mut name_tags: Query<(&mut Transform, &NameTagSprite)>,
    tag_holders: Query<(&Transform, &NameTag)>,
    camera_query: Query<&GlobalTransform, With<CameraObject>>,
) {
    let camera_transform = camera_query.iter().next().unwrap();

    for (mut transform, tag) in name_tags.iter_mut() {
        // Get the entity associated to this NameTagSprite
        let (holder_pos, name_tag) = if let Ok((holder, name_tag)) = tag_holders.get(tag.0) {
            (holder.translation, name_tag)
        } else {
            continue;
        };

        // Rotate and move NameTagSprite entity
        transform.look_at(camera_transform.translation, Vec3::unit_y());
        transform.translation = name_tag.margin * Vec3::unit_y() + holder_pos;
    }
}

// The following is old Bevy code, was deleted for some reason but
// as far as I can tell works perfectly

// Adds a `render_text` method that returns a Texture with the correct text

use ab_glyph::{self, Glyph, Point, ScaleFont};
use bevy::render::texture::{Extent3d, TextureDimension, TextureFormat};

pub trait FontExtension {
    fn render_text(
        &self,
        text: &str,
        color: Color,
        font_size: f32,
        width: usize,
        height: usize,
    ) -> Texture;
}

impl FontExtension for Font {
    fn render_text(
        &self,
        text: &str,
        color: Color,
        font_size: f32,
        width: usize,
        height: usize,
    ) -> Texture {
        let scale = ab_glyph::PxScale::from(font_size);

        let scaled_font = ab_glyph::Font::as_scaled(&self.font, scale);

        let mut glyphs = Vec::new();
        layout_paragraph(
            scaled_font,
            ab_glyph::point(0.0, 0.0),
            width as f32,
            text,
            &mut glyphs,
        );

        let color_u8 = [
            (color.r() * 255.0) as u8,
            (color.g() * 255.0) as u8,
            (color.b() * 255.0) as u8,
        ];

        let mut alpha = vec![0.0; width * height];
        for glyph in glyphs {
            if let Some(outlined) = scaled_font.outline_glyph(glyph) {
                let bounds = outlined.px_bounds();
                // Draw the glyph into the image per-pixel by using the draw closure
                outlined.draw(|x, y, v| {
                    // Offset the position by the glyph bounding box
                    // Turn the coverage into an alpha value (blended with any previous)
                    let offset_x = x as usize + bounds.min.x as usize;
                    let offset_y = y as usize + bounds.min.y as usize;
                    if offset_x >= width || offset_y >= height {
                        return;
                    }
                    alpha[offset_y * width + offset_x] = v;
                });
            }
        }

        Texture::new(
            Extent3d::new(width as u32, height as u32, 1),
            TextureDimension::D2,
            alpha
                .iter()
                .map(|a| {
                    vec![
                        color_u8[0],
                        color_u8[1],
                        color_u8[2],
                        (color.a() * a * 255.0) as u8,
                    ]
                })
                .flatten()
                .collect::<Vec<u8>>(),
            TextureFormat::Rgba8UnormSrgb,
        )
    }
}

// TODO Move to another file
fn layout_paragraph<F, SF>(
    font: SF,
    position: Point,
    max_width: f32,
    text: &str,
    target: &mut Vec<Glyph>,
) where
    F: ab_glyph::Font,
    SF: ScaleFont<F>,
{
    let v_advance = font.height() + font.line_gap();
    let mut caret = position + ab_glyph::point(0.0, font.ascent());
    let mut last_glyph: Option<Glyph> = None;
    for c in text.chars() {
        if c.is_control() {
            if c == '\n' {
                caret = ab_glyph::point(position.x, caret.y + v_advance);
                last_glyph = None;
            }
            continue;
        }
        let mut glyph = font.scaled_glyph(c);
        if let Some(previous) = last_glyph.take() {
            caret.x += font.kern(previous.id, glyph.id);
        }
        glyph.position = caret;

        last_glyph = Some(glyph.clone());
        caret.x += font.h_advance(glyph.id);

        if !c.is_whitespace() && caret.x > position.x + max_width {
            caret = ab_glyph::point(position.x, caret.y + v_advance);
            glyph.position = caret;
            last_glyph = None;
        }

        target.push(glyph);
    }
}
