// use yew::prelude::*;

// struct TodoList {
//     id: u32,
//     text: String,
//     complete: bool,
// }

// enum TodoState {
//     AddTodo,
//     CompleteTodo,
//     DeleteTodo,
// }

// #[function_component(App)]
// pub fn app() -> Html {
//     let value: UseStateHandle<bool> = use_state(|| false);
//     let buttonclick = {
//         let value = value.clone();
//         move || {
//             let valueBool = !(*value);
//             value.set(valueBool);
//         }
//     };

//     html! {
//     <main class = "w-full h-screen flex justify-center items-center">
//         <div class="w-fit h-fit flex flex-col gap-4 p-4 shadow rounded">
//             <p class="text-2xl font-semibold">{"Todo List"}</p>
//             <div class="flex justify-center items-center gap-2 ">
//                 <input class="border rounded-md p-2 border-gray-300 focus:border-black"/>
//                 <button {buttonclick} class="bg-gray-800 rounded-md p-2 text-white">{"Add Todo"}</button>
//             </div>
//         </div>
//         if *value{
//             <Todos/>
//         }
//     </main>
//     }
// }

// #[function_component(Todos)]
// pub fn todos() -> Html {
//     html! {"Hello from todo"}
//     // let todo: UseStateHandle<Vec<TodoList>> = use_state(|| vec![]);
//     // let input: UseStateHandle<_> = use_state(||String);

//     // let value: TodoList = TodoList {
//     //     id: 1,
//     //     text: "help".to_string(),
//     //     complete: false,
//     // };

//     // let setCompleted = {
//     //     let value = todo.clone();
//     //     move |_| {
//     //         let value.complete  = true;
//     //         counter.set(value);
//     //     }
//     // };

//     // todo.push(value);
//     // html! {
//     // <main class = "w-full h-screen flex justify-center items-center">

//     // </main>
//     // }
// }
