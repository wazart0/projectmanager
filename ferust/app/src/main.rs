use chrono::NaiveDateTime;
use dioxus::prelude::*;

// use tracing::*;
// use log::info;
// use serde_json;
// mod camino;

// mod models;

// mod server;
// use crate::server::{get_list_of_tasks, get_project_info};

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
    #[route("/project")]
    Project {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const PROJECT_CSS: Asset = asset!("/assets/project.css");
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        document::Link { rel: "stylesheet", href: PROJECT_CSS }
        Router::<Route> {}
    }
}

#[component]
pub fn Hero() -> Element {
    rsx! {

        {
            tracing::info!("Hero component rendered");
        }

        div { id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "doublelinks",
                div { id: "links1",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                        "💫 VSCode Extension"
                    }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                }
                div { id: "links2",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus",
                        "💫 VSCode Extension"
                    }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                }
            }
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Hero {}
        Tasks {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div { id: "blog",

            // Content
            h1 { class: "text-2xl 2xl:text-4xl", "This is blog #{id}!" }
            p { style: "font-size: 1.5rem; 2xl:font-size: 2rem;",
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components."
            }

            div { class: "flex justify-center items-center",
                a {
                    class: "text-blue-500",
                    href: "https://dioxuslabs.com/learn/0.6/",
                    "Learn Dioxus"
                }
            }

            // Navigation links
            Link { to: Route::Blog { id: id - 1 }, "Previous" }
            span { " <---> " }
            Link { to: Route::Blog { id: id + 1 }, "Next" }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Tasks() -> Element {
    let mut tasks = use_signal(String::new);
    let mut project_info = use_signal(String::new);
    use_future(move || async move {
        let response = String::new(); //get_list_of_tasks().await.unwrap();
        tasks.set(response);
    });
    use_future(move || async move {
        let response = String::new(); //get_project_info().await.unwrap();
        project_info.set(response);
    });

    rsx! {
        div { id: "echo",
            h4 { "List of tasks" }

            {tasks}
            {project_info}
        }
    }
}

enum GanttComponent {
    Table,
    Chart,
}

fn sync_scroll(component: GanttComponent) {
    match component {
        GanttComponent::Table => {
            document::eval(
                r#"
                document.getElementById("right_view").scrollTop = 
                    document.getElementById("left_view").scrollTop;
            "#,
            );
        }
        GanttComponent::Chart => {
            document::eval(
                r#"
                document.getElementById("left_view").scrollTop = 
                    document.getElementById("right_view").scrollTop;
            "#,
            );
        }
    }
}

fn get_task_cell_value(task: &communication::baselines::TaskBaseline, column: &str) -> String {
    match column {
        "task_baseline_id" => task.task_baseline_id.to_string(),
        "baseline_id" => task.baseline_id.to_string(),
        "task_id" => task.task_id.to_string(),
        "wbs" => task.wbs.clone(),
        "task_summary" => task.task_summary.clone(),
        "task_description" => match &task.task_description {
            Some(description) => description.clone(),
            None => "".to_string(),
        },
        "task_comment" => match &task.task_comment {
            Some(comment) => comment.clone(),
            None => "".to_string(),
        },
        "parent" => match task.parent {
            Some(parent) => parent.to_string(),
            None => "".to_string(),
        },
        "start" => task.start.date().to_string(),
        "start_timezone" => task.start_timezone.clone(),
        "finish" => task.finish.date().to_string(),
        "finish_timezone" => task.finish_timezone.clone(),
        _ => panic!("Invalid column: {}", column),
    }
}

fn get_resource_cell_value(
    resource: &communication::resources::Resource,
    column: &str,
    resource_types: &[communication::resources::ResourceType],
) -> String {
    match column {
        "resource_id" => resource.resource_id.to_string(),
        "name" => resource.name.clone(),
        "resource_type_id" => {
            let resource_type = resource_types
                .iter()
                .find(|rt| rt.resource_type_id == resource.resource_type_id)
                .unwrap();
            resource_type.name.clone()
        }
        "description" => match &resource.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        },
        "comment" => match &resource.comment {
            Some(comment) => comment.clone(),
            None => "".to_string(),
        },
        "cost" => match &resource.cost {
            Some(cost) => cost.to_string(),
            None => "".to_string(),
        },
        "cost_currency" => resource.cost_currency.clone(),
        "billing_frequency" => match &resource.billing_frequency {
            Some(billing_frequency) => billing_frequency.to_string(),
            None => "".to_string(),
        },
        "billing_interval" => match &resource.billing_interval {
            Some(billing_interval) => billing_interval.to_string(),
            None => "".to_string(),
        },
        "availability" => match &resource.availability {
            Some(availability) => availability.clone(),
            None => "".to_string(),
        },
        "capacity" => match &resource.capacity {
            Some(capacity) => capacity.to_string(),
            None => "".to_string(),
        },
        "capacity_unit" => match &resource.capacity_unit {
            Some(capacity_unit) => capacity_unit.clone(),
            None => "".to_string(),
        },
        "is_active" => resource.is_active.to_string(),
        _ => panic!("Invalid column: {}", column),
    }
}

fn get_resource_allocation_cell_value(
    resource_allocation: &communication::baselines::ResourceAllocation,
    column: &str,
) -> String {
    match column {
        "resource_baseline_id" => resource_allocation.resource_baseline_id.to_string(),
        "baseline_id" => resource_allocation.baseline_id.to_string(),
        "resource_id" => resource_allocation.resource_id.to_string(),
        "task_id" => resource_allocation.task_id.to_string(),
        "resource_summary" => match &resource_allocation.resource_summary {
            Some(resource_summary) => resource_summary.clone(),
            None => "".to_string(),
        },
        "task_summary" => match &resource_allocation.task_summary {
            Some(task_summary) => task_summary.clone(),
            None => "".to_string(),
        },
        "capacity_allocated" => match &resource_allocation.capacity_allocated {
            Some(capacity_allocated) => capacity_allocated.to_string(),
            None => "".to_string(),
        },
        "capacity" => match &resource_allocation.capacity {
            Some(capacity) => capacity.to_string(),
            None => "".to_string(),
        },
        "capacity_unit" => match &resource_allocation.capacity_unit {
            Some(capacity_unit) => capacity_unit.clone(),
            None => "".to_string(),
        },
        _ => panic!("Invalid column: {}", column),
    }
}

#[derive(Debug, PartialEq)]
enum View {
    Loading,
    Gantt,
    ResourcesList,
    ResourcesAllocation,
    Reporting,
    Reports,
    Settings,
}

#[component]
fn Project() -> Element {
    let mut view = use_signal(|| View::Loading);
    let mut signal_tasks: Signal<Vec<communication::baselines::TaskBaseline>> =
        use_signal(Vec::new);
    let mut signal_resources: Signal<Vec<communication::resources::Resource>> =
        use_signal(Vec::new);
    let mut signal_resource_types: Signal<Vec<communication::resources::ResourceType>> =
        use_signal(Vec::new);
    let mut signal_resource_allocations: Signal<Vec<communication::baselines::ResourceAllocation>> =
        use_signal(Vec::new);
    let mut splitter_position = use_signal(|| 50.);
    let mut project_start = use_signal(|| NaiveDateTime::MIN);
    let mut project_finish = use_signal(|| NaiveDateTime::MAX);

    let fetch_tasks = move |_| async move {
        signal_tasks.set(
            serde_json::from_str(
                &reqwest::get("http://localhost:22004/tasks?baseline_id=1")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
            )
            .unwrap(),
        );
        project_start.set(
            signal_tasks
                .read()
                .clone()
                .into_iter()
                .min_by_key(|task| task.start)
                .unwrap()
                .start,
        );
        project_finish.set(
            signal_tasks
                .read()
                .clone()
                .into_iter()
                .max_by_key(|task| task.finish)
                .unwrap()
                .finish,
        );
        view.set(View::Gantt);
    };

    let fetch_resources = move |_| async move {
        let (resources, resource_types) = bitcode::decode(
            &reqwest::get("http://localhost:22004/resources")
                .await
                .unwrap_or_else(|e| {
                    panic!("Failed to fetch resources: {}", e);
                })
                .bytes()
                .await
                .unwrap_or_else(|e| {
                    panic!("Failed to fetch resources: {}", e);
                }),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to decode resources: {}", e);
        });
        signal_resources.set(resources);
        signal_resource_types.set(resource_types);
        view.set(View::ResourcesList);
    };

    let fetch_resource_allocations = move |_| async move {
        let resource_allocations = bitcode::decode(
            &reqwest::get("http://localhost:22004/resources/allocation?baseline_id=1")
                .await
                .unwrap_or_else(|e| {
                    panic!("Failed to fetch resource allocations: {}", e);
                })
                .bytes()
                .await
                .unwrap_or_else(|e| {
                    panic!("Failed to fetch resource allocations: {}", e);
                }),
        )
        .unwrap_or_else(|e| {
            panic!("Failed to decode resource allocations: {}", e);
        });

        signal_resource_allocations.set(resource_allocations);
        view.set(View::ResourcesAllocation);
    };

    use_future(move || async move {
        signal_tasks.set(
            serde_json::from_str(
                &reqwest::get("http://localhost:22004/tasks?baseline_id=1")
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap(),
            )
            .unwrap(),
        );
        project_start.set(
            signal_tasks
                .read()
                .clone()
                .into_iter()
                .min_by_key(|task| task.start)
                .unwrap()
                .start,
        );
        project_finish.set(
            signal_tasks
                .read()
                .clone()
                .into_iter()
                .max_by_key(|task| task.finish)
                .unwrap()
                .finish,
        );
        view.set(View::Gantt);
    });

    if *view.read() == View::Loading {
        return rsx! {
            div {
                id: "loading",
                style: "display: flex; justify-content: center; align-items: center; height: 100vh; width: 100vw;",
                "Loading..."
            }
        };
    }

    rsx! {
        div { id: "project",
            div { id: "toolbar",
                button { class: "button", onclick: fetch_tasks, "Gantt" }
                button { class: "button", onclick: fetch_resources, "Resources" }
                button {
                    class: "button",
                    onclick: move |_| view.set(View::Reporting),
                    "Reporting"
                }
                button {
                    class: "button",
                    onclick: move |_| view.set(View::Reports),
                    "Reports"
                }
                button {
                    class: "button",
                    onclick: move |_| view.set(View::Settings),
                    "Settings"
                }
                span { " | " }
                if *view.read() == View::ResourcesList || *view.read() == View::ResourcesAllocation {
                    button { class: "button", onclick: fetch_resources, "List" }
                    button { class: "button", onclick: fetch_resource_allocations, "Allocation" }
                }
                if *view.read() == View::Gantt {
                    button {
                        class: "button",
                        onclick: move |_| splitter_position.set(100.),
                        "Table"
                    }
                    button {
                        class: "button",
                        onclick: move |_| splitter_position.set(0.),
                        "Gantt"
                    }
                    input {
                        r#type: "range",
                        min: "0",
                        max: "100",
                        value: splitter_position.read().to_string(),
                        oninput: move |event| splitter_position.set(event.value().parse().unwrap()),
                    }
                }
            }

            if *view.read() == View::ResourcesList {
                div { id: "full_view", style: "",
                    div { id: "resources", class: "table",
                        for (row , resource) in signal_resources.read().clone().into_iter().enumerate() {
                            for (column_index , column) in communication::resources::Resource::fields().iter().enumerate() {
                                div {
                                    class: "item",
                                    style: "grid-row: {(row+1).to_string()}; grid-column: {(column_index+1).to_string()};",
                                    "{get_resource_cell_value(&resource, column, &signal_resource_types.read())}"
                                }
                            }
                        }
                    }
                }
            }


            if *view.read() == View::ResourcesAllocation {
                div { id: "full_view", style: "",
                    div { id: "resources_allocations", class: "table",
                        for (row , resource_allocation) in signal_resource_allocations.read().clone().into_iter().enumerate() {
                            for (column_index , column) in communication::baselines::ResourceAllocation::fields().iter().enumerate() {
                                div {
                                    class: "item",
                                    style: "grid-row: {(row+1).to_string()}; grid-column: {(column_index+1).to_string()};",
                                    "{get_resource_allocation_cell_value(&resource_allocation, column)}"
                                }
                            }
                        }
                    }
                }
            }



            if *view.read() == View::Gantt {
                div {
                    id: "left_view",
                    onscroll: move |_| sync_scroll(GanttComponent::Table),
                    style: match *splitter_position.read() {
                        x if x > 98. => "width: 100vw;".to_string(),
                        x if x < 2. => "display: none;".to_string(),
                        x => format!("width: {}vw;", x),
                    },
                    div { class: "table",

                        for (row , task) in signal_tasks.read().clone().into_iter().enumerate() {
                            for (column_index , column) in communication::baselines::TaskBaseline::fields().iter().enumerate() {
                                div {
                                    class: "item",
                                    style: "grid-row: {(row+1).to_string()}; grid-column: {(column_index+1).to_string()};",
                                    "{get_task_cell_value(&task, column)}"
                                }
                            }
                        }
                    }
                }
                div {
                    id: "view_splitter",
                    style: match *splitter_position.read() {
                        x if (2.0..98.0).contains(&x) => format!("left: {}vw;", x),
                        _ => "display: none;".to_string(),
                    },
                }
                div {
                    id: "right_view",
                    onscroll: move |_| sync_scroll(GanttComponent::Chart),
                    style: match *splitter_position.read() {
                        x if x < 2. => "width: 100vw; left: 0vw;".to_string(),
                        x if x > 98. => "display: none;".to_string(),
                        x => format!("left: {}vw; width: {}vw;", x + 0.2, 99.8 - x),
                    },
                    div { class: "gantt_chart",
                        for (row , task) in signal_tasks.read().clone().into_iter().enumerate() {
                            div {
                                class: "item",
                                style: "grid-row: {(row+1).to_string()}; width: 110rem;",
                                div {
                                    width: ((task.finish - task.start).num_seconds() as f64 * 100.
                                        / (*project_finish.read() - *project_start.read()).num_seconds() as f64)
                                        .to_string() + "rem",
                                    left: (((task.start - *project_start.read()).num_seconds() as f64 * 100.
                                        / (*project_finish.read() - *project_start.read()).num_seconds() as f64) + 5.)
                                        .to_string() + "rem",
                                    style: "background-color: green; position: relative; height: 100%;
                                        box-sizing: border-box; border-bottom: 0.2rem solid black; border-top: 0.2rem solid black;",
                                    {
                                        tracing::debug!(
                                            "task: {}, start: {}, duration: {}", task.task_summary, (task.start - *
                                            project_start.read()).num_seconds() as f64 * 100. / (* project_finish.read()
                                            - * project_start.read()).num_seconds() as f64, (task.finish - task.start)
                                            .num_seconds() as f64 * 100. / (* project_finish.read() - * project_start
                                            .read()).num_seconds() as f64,
                                        )
                                    }
                                }
                                                        // div {
                            //     style: "position: relative; height: 100%; z-index: 10; left: 10rem",
                            //     "{task.name}"
                            // }
                            }
                        }
                    }
                }
            }
        }
    }
}
