use crate::components::context::{extend_context_with_overlay, LeafletMapContext};
use crate::components::path_options::{FillRule, LineCap, LineJoin};
use crate::components::position::Position;
use crate::core::LeafletMaybeSignal;
use crate::{
    setup_layer_leaflet_option, setup_layer_leaflet_option_ref, LayerEvents, MouseEvents,
    PopupEvents, TooltipEvents,
};
use leaflet::CircleOptions;
use leptos::*;

#[component(transparent)]
pub fn Circle(
    #[prop(into)] center: MaybeSignal<Position>,
    #[prop(into, optional)] stroke: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] color: LeafletMaybeSignal<String>,
    #[prop(into, optional)] weight: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] interactive: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] line_cap: LeafletMaybeSignal<LineCap>,
    #[prop(into, optional)] line_join: LeafletMaybeSignal<LineJoin>,
    #[prop(into, optional)] dash_array: LeafletMaybeSignal<String>,
    #[prop(into, optional)] dash_offset: LeafletMaybeSignal<String>,
    #[prop(into, optional)] fill: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] fill_color: LeafletMaybeSignal<String>,
    #[prop(into, optional)] fill_opacity: LeafletMaybeSignal<f64>,
    #[prop(into, optional)] fill_rule: LeafletMaybeSignal<FillRule>,
    #[prop(into, optional)] bubbling_mouse_events: LeafletMaybeSignal<bool>,
    #[prop(into, optional)] class_name: LeafletMaybeSignal<String>,
    #[prop(into, optional)] mouse_events: MouseEvents,
    #[prop(into, optional)] layer_events: LayerEvents,
    #[prop(into, optional)] popup_events: PopupEvents,
    #[prop(into, optional)] tooltip_events: TooltipEvents,

    #[prop(into)] radius: LeafletMaybeSignal<f64>,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let overlay_context = extend_context_with_overlay();
    let overlay = store_value(None::<leaflet::Circle>);

    let color_clone = color.clone();
    let fill_color_clone = fill_color.clone();
    create_effect(move |_| {
        if let Some(map) = use_context::<LeafletMapContext>()
            .expect("map context")
            .map()
        {
            let mut options = CircleOptions::new();
            setup_layer_leaflet_option!(stroke, options);
            setup_layer_leaflet_option_ref!(color, options);
            setup_layer_leaflet_option!(weight, options);
            setup_layer_leaflet_option!(radius, options);
            setup_layer_leaflet_option!(opacity, options);
            setup_layer_leaflet_option!(interactive, options);
            setup_layer_leaflet_option_ref!(line_cap, options);
            setup_layer_leaflet_option_ref!(line_join, options);
            setup_layer_leaflet_option_ref!(dash_array, options);
            setup_layer_leaflet_option_ref!(dash_offset, options);
            setup_layer_leaflet_option!(fill, options);
            setup_layer_leaflet_option_ref!(fill_color, options);
            setup_layer_leaflet_option!(fill_opacity, options);
            setup_layer_leaflet_option_ref!(fill_rule, options);
            setup_layer_leaflet_option!(bubbling_mouse_events, options);
            setup_layer_leaflet_option_ref!(class_name, options);
            let circle =
                leaflet::Circle::new_with_options(&center.get_untracked().into(), &options);

            mouse_events.setup(&circle);
            popup_events.setup(&circle);
            tooltip_events.setup(&circle);
            layer_events.setup(&circle);

            circle.addTo(&map);
            overlay_context.set_container(&circle);
            overlay.set_value(Some(circle));
        };
    });

    // let radius_stop = watch(
    //     move || radius.get(),
    //     move |radius, _, _| {
    //         if let Some(polygon) = overlay.get_value() {
    //             polygon.setRadius(*radius);
    //         }
    //     },
    //     false,
    // );

    let stroke_stop = watch(
        move || stroke.get(),
        move |stroke, _, _| {
            if let (Some(stroke), Some(overlay)) = (stroke, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.stroke(*stroke);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    let color_stop = watch(
        move || color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.color(color);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    let fill_color_stop = watch(
        move || fill_color_clone.get(),
        move |color, _, _| {
            if let (Some(color), Some(overlay)) = (color, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.fill_color(color);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    let opacity_stop = watch(
        move || opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.opacity(*opacity);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    let fill_opacity_stop = watch(
        move || fill_opacity.get(),
        move |opacity, _, _| {
            if let (Some(opacity), Some(overlay)) = (opacity, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.fill_opacity(*opacity);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    let weight_stop = watch(
        move || weight.get(),
        move |weight, _, _| {
            if let (Some(weight), Some(overlay)) = (weight, overlay.get_value()) {
                let mut options = CircleOptions::new();
                options.weight(*weight);
                overlay.setStyle(&options);
            }
        },
        false,
    );

    on_cleanup(move || {
        // radius_stop();
        stroke_stop();
        color_stop();
        fill_color_stop();
        opacity_stop();
        fill_opacity_stop();
        weight_stop();
        if let Some(overlay) = overlay.get_value() {
            overlay.remove();
        }
    });

    children.map(|child| child())
}
