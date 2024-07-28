#![allow(dead_code)]
use bevy::{color::palettes::css, prelude::*};

use crate::*;

pub fn c_full_screen(bundle: &mut NodeBundle) {
    bundle.style.position_type = PositionType::Absolute;
    bundle.style.top = Val::Px(0.);
    bundle.style.left = Val::Px(0.);
    bundle.style.bottom = Val::Px(0.);
    bundle.style.right = Val::Px(0.);
}

pub fn c_border_1(bundle: &mut NodeBundle) {
    bundle.style.border = UiRect::all(Val::Px(1.));
}

pub fn c_border_yellow(bundle: &mut NodeBundle) {
    c_border_1.apply(bundle);
    bundle.border_color = css::YELLOW.into();
}

pub fn c_border_white(bundle: &mut NodeBundle) {
    c_border_1.apply(bundle);
    bundle.border_color = css::WHITE.into();
}

pub fn c_border_red(bundle: &mut NodeBundle) {
    c_border_1.apply(bundle);
    bundle.border_color = css::RED.into();
}

pub fn c_border_green(bundle: &mut NodeBundle) {
    c_border_1.apply(bundle);
    bundle.border_color = css::GREEN.into();
}

pub fn c_bg_dark_gray(bundle: &mut NodeBundle) {
    bundle.background_color = css::DARK_GRAY.into();
}

pub fn c_bg_red(bundle: &mut NodeBundle) {
    bundle.background_color = css::RED.into();
}

pub fn c_bg_green(bundle: &mut NodeBundle) {
    bundle.background_color = css::GREEN.into();
}

pub fn c_bg_blue(bundle: &mut NodeBundle) {
    bundle.background_color = css::BLUE.into();
}

pub fn c_bg_white(bundle: &mut NodeBundle) {
    bundle.background_color = css::WHITE.into();
}

pub fn c_row_100_100(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Row;
    bundle.style.width = Val::Percent(100.);
    bundle.style.height = Val::Percent(100.);
}

pub fn c_row(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Row;
}

pub fn c_abs_stretch(bundle: &mut NodeBundle) {
    bundle.style.overflow = Overflow::clip();
    bundle.style.max_height = Val::Percent(100.);
    bundle.style.position_type = PositionType::Absolute;
    bundle.style.top = Val::Px(0.);
    bundle.style.left = Val::Px(0.);
    bundle.style.bottom = Val::Px(0.);
    bundle.style.right = Val::Px(0.);
}

pub fn c_overflow_clip(bundle: &mut NodeBundle) {
    bundle.style.overflow = Overflow::clip();
}

pub fn c_overflow_clip_x(bundle: &mut NodeBundle) {
    bundle.style.overflow = Overflow::clip_x();
}

pub fn c_fill_stretch(bundle: &mut NodeBundle) {
    bundle.style.flex_grow = 1.;
    bundle.style.width = Val::Percent(100.);
    bundle.style.justify_self = JustifySelf::Stretch;
}

pub fn c_mb_5(bundle: &mut NodeBundle) {
    bundle.style.margin.bottom = Val::Px(5.);
}

pub fn c_pt_5(bundle: &mut NodeBundle) {
    bundle.style.padding.top = Val::Px(5.);
}

pub fn c_mt_auto(bundle: &mut NodeBundle) {
    bundle.style.margin.top = Val::Auto;
}

pub fn c_scrollable_parent(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Column;
    bundle.style.align_self = AlignSelf::Stretch;
    bundle.style.height = Val::Percent(100.);
    bundle.style.width = Val::Percent(100.);
}

pub fn c_flex_grow(bundle: &mut NodeBundle) {
    bundle.style.flex_grow = 1.;
}

pub fn c_row_w_100(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Row;
    bundle.style.width = Val::Percent(100.);
}

pub fn c_row_w_50(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Row;
    bundle.style.width = Val::Percent(50.);
}

pub fn c_col_50(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Column;
    bundle.style.width = Val::Percent(50.);
}

pub fn c_align_end(bundle: &mut NodeBundle) {
    bundle.style.align_items = AlignItems::End;
}

pub fn c_align_center(bundle: &mut NodeBundle) {
    bundle.style.align_items = AlignItems::Center;
}

pub fn c_align_stretch(bundle: &mut NodeBundle) {
    bundle.style.align_items = AlignItems::Stretch;
}

pub fn c_row_reverse(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::RowReverse;
}

pub fn c_justify_stretch(bundle: &mut NodeBundle) {
    bundle.style.justify_content = JustifyContent::Stretch;
}

pub fn c_justify_end(bundle: &mut NodeBundle) {
    bundle.style.justify_content = JustifyContent::End;
}

pub fn c_pr_5(bundle: &mut NodeBundle) {
    bundle.style.padding.right = Val::Px(5.);
}

pub fn c_pl_5(bundle: &mut NodeBundle) {
    bundle.style.padding.left = Val::Px(5.);
}

pub fn c_col(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Column;
}

pub fn c_justify_items_stretch(bundle: &mut NodeBundle) {
    bundle.style.justify_items = JustifyItems::Stretch;
}

pub fn c_col_w_100(bundle: &mut NodeBundle) {
    bundle.style.flex_direction = FlexDirection::Column;
    bundle.style.width = Val::Percent(100.);
}

pub fn c_display_none(bundle: &mut NodeBundle) {
    bundle.style.display = Display::None;
}

pub fn c_visibility_hidden(bundle: &mut NodeBundle) {
    bundle.visibility = Visibility::Hidden;
}

pub fn c_wh_100(bundle: &mut NodeBundle) {
    bundle.style.width = Val::Percent(100.);
    bundle.style.height = Val::Percent(100.);
}

pub fn c_pb_200(bundle: &mut NodeBundle) {
    bundle.style.padding.bottom = Val::Px(200.);
}

pub fn c_stretch_center(bundle: &mut NodeBundle) {
    bundle.style.width = Val::Percent(100.);
    bundle.style.height = Val::Percent(100.);
    bundle.style.align_items = AlignItems::Center;
    bundle.style.justify_content = JustifyContent::Center;
}

pub fn c_center(bundle: &mut NodeBundle) {
    bundle.style.align_items = AlignItems::Center;
    bundle.style.justify_content = JustifyContent::Center;
}

pub fn c_justify_center(bundle: &mut NodeBundle) {
    bundle.style.justify_content = JustifyContent::Center;
}

pub fn c_border_bottom(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.border.bottom = Val::Px(size);
    }
}

pub fn c_border_left(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.border.left = Val::Px(size);
    }
}

pub fn c_border_right(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.border.right = Val::Px(size);
    }
}

pub fn c_border_color(color: impl Into<BorderColor>) -> impl Fn(&mut NodeBundle) {
    let border_color: BorderColor = color.into();
    move |b: &mut NodeBundle| {
        b.border_color = border_color;
    }
}

pub fn c_background_color(color: impl Into<BackgroundColor>) -> impl Fn(&mut NodeBundle) {
    let bg: BackgroundColor = color.into();
    move |b: &mut NodeBundle| {
        b.background_color = bg;
    }
}

pub fn c_padding_bottom(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.bottom = Val::Px(size);
    }
}

pub fn c_padding(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding = UiRect::all(Val::Px(size));
    }
}

pub fn c_padding_top(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.top = Val::Px(size);
    }
}

pub fn c_padding_right(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.right = Val::Px(size);
    }
}

pub fn c_padding_vertical(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.top = Val::Px(size);
        b.style.padding.bottom = Val::Px(size);
    }
}

pub fn c_padding_horizontal(size: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.left = Val::Px(size);
        b.style.padding.right = Val::Px(size);
    }
}

pub fn c_padding_axes(x: f32, y: f32) -> impl Fn(&mut NodeBundle) {
    move |b: &mut NodeBundle| {
        b.style.padding.top = Val::Px(y);
        b.style.padding.bottom = Val::Px(y);
        b.style.padding.left = Val::Px(x);
        b.style.padding.right = Val::Px(x);
    }
}

pub fn c_justify_between(bundle: &mut NodeBundle) {
    bundle.style.justify_content = JustifyContent::SpaceBetween;
}

pub fn c_h_100(bundle: &mut NodeBundle) {
    bundle.style.height = Val::Percent(100.);
}

pub fn c_w_100(bundle: &mut NodeBundle) {
    bundle.style.width = Val::Percent(100.);
}

pub fn c_node_square(size: f32) -> impl Fn(&mut NodeBundle) {
    c_node_size(size, size)
}

pub fn c_node_size(width: f32, height: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.width = Val::Px(width);
        bundle.style.height = Val::Px(height);
    }
}

pub fn c_image_square(size: f32) -> impl Fn(&mut ImageBundle) {
    c_image_size(size, size)
}

pub fn c_image_size(width: f32, height: f32) -> impl Fn(&mut ImageBundle) {
    move |bundle: &mut ImageBundle| {
        bundle.style.width = Val::Px(width);
        bundle.style.height = Val::Px(height);
    }
}

pub fn c_border_thickness(thickness: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.border = UiRect::all(Val::Px(thickness));
    }
}

pub fn c_width(width: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.width = Val::Px(width);
    }
}

pub fn c_width_percent(width: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.width = Val::Percent(width);
    }
}

pub fn c_height(height: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.height = Val::Px(height);
    }
}

pub fn c_height_percent(height: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.height = Val::Percent(height);
    }
}

pub fn c_max_width(width: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.width = Val::Px(width);
    }
}

pub fn c_max_height(height: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.height = Val::Px(height);
    }
}

pub fn c_min_width(width: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.min_width = Val::Px(width);
    }
}

pub fn c_min_height(height: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.min_height = Val::Px(height);
    }
}

pub fn c_margin_top(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.margin.top = Val::Px(px);
    }
}

pub fn c_margin_left(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.margin.left = Val::Px(px);
    }
}

pub fn c_margin_right(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.margin.right = Val::Px(px);
    }
}

pub fn c_margin_bottom(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.margin.bottom = Val::Px(px);
    }
}

pub fn c_margin(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.margin = UiRect::all(Val::Px(px));
    }
}

pub fn c_z_index_local(z_index: i32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.z_index = ZIndex::Local(z_index);
    }
}

pub fn c_top(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.top = Val::Px(px);
    }
}

pub fn c_right(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.right = Val::Px(px);
    }
}

pub fn c_bottom(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.bottom = Val::Px(px);
    }
}

pub fn c_left(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.style.left = Val::Px(px);
    }
}

pub fn c_scale(scale: Vec3) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.transform.scale = scale;
    }
}

pub fn c_position_absolute(b: &mut NodeBundle) {
    b.style.position_type = PositionType::Absolute;
}

/// System that spawns the settings menu
pub fn scrollable_wrapper(
    parent: &mut ChildBuilder,
    inner_class: impl ClassBuilder<NodeBundle>,
    children: impl FnOnce(&mut ChildBuilder),
) {
    node(parent, (), c_fill_stretch).with_children(|p| {
        node(p, (), c_abs_stretch).with_children(|p| {
            node(p, (), (c_scrollable_parent, inner_class)).with_children(children);
        });
    });
}

pub fn c_border_none(b: &mut NodeBundle) {
    b.style.border = UiRect::ZERO;
    b.border_color = Color::NONE.into();
}

pub fn c_max_height_100(bundle: &mut NodeBundle) {
    bundle.style.max_height = Val::Percent(100.);
}

pub fn c_ml_auto(bundle: &mut NodeBundle) {
    bundle.style.margin.left = Val::Auto;
}

pub fn c_mx_auto(bundle: &mut NodeBundle) {
    bundle.style.margin.left = Val::Auto;
    bundle.style.margin.right = Val::Auto;
}

pub fn c_d_none(bundle: &mut NodeBundle) {
    bundle.style.display = Display::None;
}

pub fn c_border_radius(px: f32) -> impl Fn(&mut NodeBundle) {
    move |bundle: &mut NodeBundle| {
        bundle.border_radius = BorderRadius::all(Val::Px(px));
    }
}
