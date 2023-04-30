import {invoke} from '@tauri-apps/api/tauri';
import {listen} from '@tauri-apps/api/event';



let unListen = listen("react_hello", (arg) => {
    console.log(arg);
});
export function hello() {
    return invoke('test_hello', {user: {name: 'hello', password: 'world', array: ["1", "2", "3"]}});
}