import {invoke, event} from "@tauri-apps/api";


export async function call<T>(api: string, arg?: any) {
    console.log("API:", api, " DATA:", arg);
    return invoke<T>(api, arg);
}

export function listen<T>(key: string, handler: event.EventCallback<T>) {
    return event.listen<T>(key, handler).then((close) => {
        return () => {
            close();
        }
    });
}
