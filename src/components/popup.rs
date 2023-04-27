use super::{LeafletMapContext, LeafletOverlayContainerContext, Position};

use leptos::*;
use leptos::html::Div;
use leptos::leptos_dom::CoreComponent::DynChild;
use leptos::leptos_dom::{HydrationCtx, is_server};
use wasm_bindgen::prelude::*;
use web_sys::HtmlDivElement;

#[component]
pub fn Popup(
    cx: Scope,
    #[prop(into, optional)] position: MaybeSignal<Position>,
    children: Children,
) -> impl IntoView {
    let map_context = use_context::<LeafletMapContext>(cx).expect("Map context not found");
    let overlay_context = use_context::<LeafletOverlayContainerContext>(cx);

    // Render popup content to a html element
    let content = create_node_ref::<Div>(cx);
    // let content = view! {cx, <div>{children(cx)}</div>};
    create_effect(cx, move |_| {
        log!("Popup context {:?}", map_context);
        let inner_content = content.clone();
        if let Some(overlay_context) = overlay_context {
            log!("we are in marker");
            if let (Some(marker), Some(_map)) = (
                overlay_context.container::<leaflet::Layer>(),
                map_context.map(),
            ) {
                log!("Adding popup");
                let options = leaflet::PopupOptions::default();
                let popup = leaflet::Popup::new(&options, Some(marker.unchecked_ref()));
                let content = inner_content.get_untracked().expect("content ref");
                let html_view: &JsValue = content.unchecked_ref();
                popup.setContent(html_view);
                marker.bindPopup(&popup);
                on_cleanup(cx, move || {
                    popup.remove();
                });
            }
        } else if let Some(map) = map_context.map() {
            log!("Adding popup");
            let options = leaflet::PopupOptions::default();
            let popup = leaflet::Popup::new_with_lat_lng(&position().into(), &options);
            let content = inner_content.get_untracked().expect("content ref");
            let html_view: &JsValue = content.unchecked_ref();
            popup.setContent(html_view);
            popup.openOn(&map);
            on_cleanup(cx, move || {
                popup.remove();
            });
        }
    });
    
    view! {cx, <div style="visibility:collapse"><div _ref=content>{children(cx)}</div></div> }
}
