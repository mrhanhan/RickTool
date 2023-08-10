import {invoke} from "@tauri-apps/api";


export async function call(api: string, arg?: any) {
    return invoke(api, arg);
}
