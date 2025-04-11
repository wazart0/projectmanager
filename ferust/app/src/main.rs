use dioxus::prelude::*;
use tracing::*;
// use log::info;
// use serde_json;
// mod camino;

// mod models;
use common;

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

        {tracing::info!("Hero component rendered");}

        div {
            id: "hero",
            img { src: HEADER_SVG, id: "header" }
            div { id: "doublelinks",
                div { 
                    id: "links1",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
                    a { href: "https://discord.gg/XgGxMSkvUM", "👋 Community Discord" }
                }
                div { 
                    id: "links2",
                    a { href: "https://dioxuslabs.com/learn/0.6/", "📚 Learn Dioxus" }
                    a { href: "https://dioxuslabs.com/awesome", "🚀 Awesome Dioxus" }
                    a { href: "https://github.com/dioxus-community/", "📡 Community Libraries" }
                    a { href: "https://github.com/DioxusLabs/sdk", "⚙️ Dioxus Development Kit" }
                    a { href: "https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus", "💫 VSCode Extension" }
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
        div {
            id: "blog",

            // Content
            h1 { 
                class: "text-2xl 2xl:text-4xl",
                "This is blog #{id}!" 
            }
            p { 
                style: "font-size: 1.5rem; 2xl:font-size: 2rem;",
                "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." 
            }

            div {
                class: "flex justify-center items-center",
                a {
                    class: "text-blue-500",
                    href: "https://dioxuslabs.com/learn/0.6/",
                    "Learn Dioxus"
                }
            }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Tasks() -> Element {
    let mut tasks = use_signal(|| String::new());
    let mut project_info = use_signal(|| String::new());
    use_future(move || async move {
        let response = String::new(); //get_list_of_tasks().await.unwrap();
        tasks.set(response);
    });
    use_future(move || async move {
        let response = String::new(); //get_project_info().await.unwrap();
        project_info.set(response);
    });


    rsx! {
        div {
            id: "echo",
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
            document::eval(r#"
                document.getElementById("right_view").scrollTop = 
                    document.getElementById("left_view").scrollTop;
            "#);
        }
        GanttComponent::Chart => {
            document::eval(r#"
                document.getElementById("left_view").scrollTop = 
                    document.getElementById("right_view").scrollTop;
            "#);
        }
    }
}



fn get_cell_value(task: &common::models::Task, column: &str) -> String {
    match column {
        "id" => task.id.to_string(),
        "wbs" => task.wbs.clone(),
        "name" => task.name.clone(),
        "description" => match &task.description {
            Some(description) => description.clone(),
            None => "".to_string(),
        },
        "parent" => match task.parent {
            Some(parent) => parent.to_string(),
            None => "".to_string(),
        },
        "begin_month" => match task.begin_month {
            Some(begin_month) => begin_month.to_string(),
            None => "".to_string(),
        },
        "end_month" => match task.end_month {
            Some(end_month) => end_month.to_string(),
            None => "".to_string(),
        },
        "planned_work_pm" => match task.planned_work_pm {
            Some(planned_work_pm) => planned_work_pm.to_string(),
            None => "".to_string(),
        },
        "planned_team_cost_eur" => match task.planned_team_cost_eur {
            Some(planned_team_cost_eur) => planned_team_cost_eur.to_string(),
            None => "".to_string(),
        },
        "planned_other_cost_eur" => match task.planned_other_cost_eur {
            Some(planned_other_cost_eur) => planned_other_cost_eur.to_string(),
            None => "".to_string(),
        },
        _ => panic!("Invalid column: {}", column),
    }
}

#[derive(Debug, PartialEq)]
enum View {
    Gantt,
    Resources,
    Reports,
    Settings,
}

#[component]
fn Project() -> Element {
    let mut view = use_signal(|| View::Gantt);
    let mut signal_tasks = use_signal(|| Vec::new());
    let mut signal_team: Signal<Vec<common::models::TeamMember>> = use_signal(|| Vec::new());
    let mut splitter_position = use_signal(|| 50.);
    
    use_future(move || async move {
        signal_tasks.set(bitcode::decode(
            &reqwest::get("http://localhost:22004/tasks")
            .await.unwrap().bytes().await.unwrap()).unwrap());
    });
    // use_future(move || async move {
    //     signal_team.set(bitcode::decode(
    //         &reqwest::get("http://localhost:22004/team")
    //         .await.unwrap().bytes().await.unwrap()).unwrap());
    // });



    rsx! {
        div {
            id: "project",
            div {
                id: "toolbar",
                button {
                    onclick: move |_| view.set(View::Gantt),
                    "Gantt"
                }
                button {
                    onclick: move |_| view.set(View::Resources),
                    "Resources"
                }
                button {
                    onclick: move |_| view.set(View::Reports),
                    "Reports"
                }
                button {
                    onclick: move |_| view.set(View::Settings),
                    "Settings"
                }
                span { " | " }
                if *view.read() == View::Gantt {
                    button {
                        onclick: move |_| splitter_position.set(100.),
                        "Table"
                    }
                    button {
                        onclick: move |_| splitter_position.set(0.),
                        "Gantt"
                    }
                    input {
                        type: "range",
                        min: "0",
                        max: "100",
                        value: splitter_position.read().to_string(),
                        oninput: move |event| splitter_position.set(event.value().parse().unwrap()),
                    }
                }
            }
            

            if *view.read() == View::Resources {
                div {
                    id: "resources",
                    for (row, team_member) in signal_team.read().clone().into_iter().enumerate() {
                        div {
                            style: "grid-row: {(row+1).to_string()};",
                            "{team_member.user_name} {team_member.user_last_name}"
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
                        
                        div {
                            class: "gantt_table",

                            for (row, task) in signal_tasks.read().clone().into_iter().enumerate() {
                                for (column_index, column) in common::models::COLUMNS.iter().enumerate() {
                                    div { 
                                        class: "item",
                                        style: "grid-row: {(row+1).to_string()}; grid-column: {(column_index+1).to_string()};",
                                        "{get_cell_value(&task, column)}"
                                    }
                                }
                            }
                        }
                    }
                    div { 
                        id: "view_splitter",
                        style: match *splitter_position.read() {
                            x if 2. <= x && x <= 98. => format!("left: {}vw;", x), 
                            _ => "display: none;".to_string(),
                        }
                    }
                    div {
                        id: "right_view",
                        onscroll: move |_| sync_scroll(GanttComponent::Chart),
                        style: match *splitter_position.read() {
                            x if x < 2. => "width: 100vw; left: 0vw;".to_string(),
                            x if x > 98. => "display: none;".to_string(),
                            x => format!("left: {}vw; width: {}vw;", x + 0.2, 99.8 - x),
                        },
                        div {
                            class: "gantt_chart",

                            for (row, task) in signal_tasks.read().clone().into_iter().enumerate() {
                                div {
                                    // class: "gantt_chart_row",
                                    style: "grid-row: {(row+1).to_string()};",
                                    width: "100rem",
                                    if task.begin_month.is_some() && task.end_month.is_some() {
                                        div {
                                            width: ((task.end_month.unwrap() - task.begin_month.unwrap())* 100 / 29).to_string() + "rem",
                                            left: (task.begin_month.unwrap() * 100 / 29).to_string() + "rem",
                                            style: "background-color: green; position: relative; height: 100%; 
                                            box-sizing: border-box; border-bottom: 0.2rem solid black; border-top: 0.2rem solid black;",
                                        }
                                    } else {
                                        div { style: "position: relative; height: 100%;" }
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



