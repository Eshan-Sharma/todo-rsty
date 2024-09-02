use web_sys;
use yew::prelude::*;
#[derive(PartialEq, Clone)]
struct TodoItem {
    id: u32,
    text: String,
    completed: bool,
}

#[function_component(App)]
pub fn app() -> Html {
    let todos = use_state(|| Vec::<TodoItem>::new());
    let new_todo = use_state(|| String::new());

    let on_input = {
        let new_todo = new_todo.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            new_todo.set(input.value());
        })
    };

    let on_add = {
        let todos = todos.clone();
        let new_todo = new_todo.clone();
        Callback::from(move |_| {
            let mut current_todos = (*todos).clone();
            current_todos.push(TodoItem {
                id: current_todos.len() as u32,
                text: (*new_todo).clone(),
                completed: false,
            });
            todos.set(current_todos);
            new_todo.set(String::new());
        })
    };

    html! {
        <main class="w-full h-screen flex justify-center items-center">
            <div class="w-fit h-fit flex flex-col gap-4 p-4 shadow rounded">
                <p class="text-2xl font-semibold">{"Todo List"}</p>
                <div class="flex justify-center items-center gap-2">
                    <input
                        class="border rounded-md p-2 border-gray-300 focus:border-black"
                        value={(*new_todo).clone()}
                        oninput={on_input}
                    />
                    <button onclick={on_add} class="bg-gray-800 rounded-md p-2 text-white">
                        {"Add Todo"}
                    </button>
                </div>
                <div class="max-h-32 min-h-24 overflow-y-scroll scroll-smooth">
                    <Todos todos={(*todos).clone()} />
                </div>
            </div>
        </main>
    }
}

#[derive(Properties, PartialEq)]
struct TodosProps {
    todos: Vec<TodoItem>,
}

#[function_component(Todos)]
fn todos(props: &TodosProps) -> Html {
    html! {
        <ul class="list-disc pl-5 ">
            {props.todos.iter().map(|todo| html! {
                <li key={todo.id}>
                    {&todo.text}
                </li>
            }).collect::<Html>()}
        </ul>
    }
}
