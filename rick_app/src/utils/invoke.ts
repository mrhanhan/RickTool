import {invoke} from "@tauri-apps/api";


export async function call<T>(api: string, arg?: any) {
    return invoke<T>(api, arg);
}
