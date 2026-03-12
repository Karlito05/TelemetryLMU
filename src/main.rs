extern crate core;

use std::ptr::read;
use std::time::Duration;
use dioxus::prelude::*;
use crate::telemetry::{get_mmap, update_telemetry, SharedMemoryObjectOut};
use memmap2::Mmap;
use graph_view::{GraphView, GraphViewDataType};

mod telemetry;
mod graph_view;

pub static MMAP: GlobalSignal<Mmap> = Signal::global(|| get_mmap("/dev/shm/LMU_Data"));
pub static TELEMETRY: GlobalSignal<SharedMemoryObjectOut> = Signal::global(|| update_telemetry(&MMAP.read()).unwrap());

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");



fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {

    use_future(move || async move {
        loop {
            tokio::time::sleep(Duration::from_millis(16)).await;

            *TELEMETRY.write() = update_telemetry(&MMAP.read()).unwrap();
        }
    });

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        div { class: "m-10",
            GraphView {
                data_type: GraphViewDataType::Rpm(0),
                width: 1000,
                height: 200,
            }
            div { class: "m-5"}
            GraphView {
                data_type: GraphViewDataType::Delta(0, 10.0),
                width: 1000,
                height: 200,
           }
            div { class: "m-5"}
            GraphView {
                data_type: GraphViewDataType::Speed(0),
                width: 1000,
                height: 200,
            }
            div { class: "m-5"}
            GraphView {
                data_type: GraphViewDataType::Throttle(0),
                width: 1000,
                height: 200,
            }

                        div { class: "m-5"}
            GraphView {
                data_type: GraphViewDataType::Pedals(0),
                width: 1000,
                height: 200,
            }
        }
    }
}