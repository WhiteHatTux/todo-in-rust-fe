use seed::{*, prelude::*};
use uuid::Uuid;

mod utils;

enum Msg {
    Fetch,
    Received(Vec<Todo>),
}

#[derive(serde::Deserialize)]
struct TodoList {
    todos: Vec<Todo>,
}

#[derive(serde::Deserialize, Default, Debug)]
struct Todo {
    id: Uuid,
    title: String,
    content: String,
}

#[derive(Clone)]
struct Stuff {
    name: String,
}

#[derive(Default, Debug)]
struct Model {
    todos: Option<Vec<Todo>>,
}

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model::default()
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Fetch => {
            orders.skip();
            orders.perform_cmd(async {
                let response = fetch("http://localhost:8083/todos")
                    .await
                    .expect("Backend not available");

                let todo_list = response
                    .check_status()
                    .expect("status check failed")
                    .json::<TodoList>()
                    .await
                    .expect("deserialization failed");

                Msg::Received(todo_list.todos)
            });
        }
        Msg::Received(todo_list) => model.todos = Some(todo_list),
    }
}


fn print_todos(todos: &Option<Vec<Todo>>) -> Node<Msg> {
    let mut total = Vec::new();
    match todos {
        None => (),
        Some(todo_list) => {
            todo_list.iter().for_each(|todo| {
                total.push(div![input![attrs!{
                    At::Type => "checkbox"
                }],format!("Tarea: {}, Descripcion: {}", todo.title, todo.content).as_str()]);
            })
        }
    }
    div![total]
}

fn view(model: &Model) -> impl IntoNodes<Msg> {
    div![
        button![ev(Ev::Click, |_| Msg::Fetch),"Fetch todos"],
        "todos: ",
        print_todos(&model.todos),
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    // Mount to the root element with id "app".
    // You can pass also `web_sys::Element` or `web_sys::HtmlElement` as a root element.
    // It's NOT recommended to mount into body or into elements which contain scripts.
    App::start("app", init, update, view);
}
