import { invoke } from "@tauri-apps/api";

/**
 * 调用数据
 * @param operate 
 * @param data 
 */
export async function api(operate: string, data: any): Promise<string> {
    return invoke('api', {operate, data});
}
/**
 * 调用数据
 * @param operate 
 * @param data 
 */
export async function api_json(operate: string, data: any): Promise<any> {
    return invoke('api', {operate, data: JSON.stringify(data)})
        .then(resp => {
            return JSON.parse(resp as string);
    });
}