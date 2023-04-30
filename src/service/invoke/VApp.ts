/**
 * 运行App
 */
import {VApp} from "../../store/store";
import {invoke} from "@tauri-apps/api";

/**
 * 运行App
 * @param app {VApp}
 * @param id ID
 */
export function invokeRunApp(app: VApp, id: string) {

    const commitApp = {
        name: app.name  || '',
        target: app.target || '',
        target_type: app.targetType,
        shell: app.shell,
        dir: app.dir || '',
        sudo: app.sudo || false,
        args: (app.args || []).map(t => ({opt: t.opt, value: t.value  || '', input_type: t.inputType})),
        environment: (app.environment || []).map( t => ({key: t.key || '', value: t.value || ''})),
    };
    return invoke('run_app', {app: commitApp, windowId: id});
}