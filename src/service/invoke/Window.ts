import {invoke} from '@tauri-apps/api/tauri';
import { genId } from '../../utils/id';



export function createWindow(label: string, target: string, width: number, height: number) {
    const id = genId();
    return new Promise<string>((resolve, reject) => {
        invoke('create_window', {id, label, target, width, height}).then(() => resolve(id)).catch(reject);
    });
}

export function closeWindow(id: string) {
    return invoke('close_window', {id});
}

export function sendMessage(id: string, event: string, payload: string) {
    return invoke('send_message', {id, event, payload});
}
