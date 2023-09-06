import {invoke} from "@tauri-apps/api";


export async function call<T>(api: string, arg?: any) {
    console.log("API:", api, " DATA:", arg);
    return invoke<T>(api, arg);
}
