import {App, AppArgs, AppExt, AppStart} from "./app-model";
import {AppStartProps} from "./items/app-start";
import {ArgType} from "../../component/arg-input/arg-data";
import {getNow} from "../../utils/uid";
import {start} from "repl";


export const AppType = [
    {
        value: 100,
        label: '可执行程序'
    },
    {
        value: 200,
        label: 'Java程序'
    },
    {
        value: 201,
        label: 'Python程序'
    },
    {
        value: 202,
        label: 'NodeJS'
    },
    {
        value: 300,
        label: '网页'
    }
]

export type AppFormModel =  Partial<App> & Record<string, any>;

export function processForm(val: AppFormModel): App {
    // 处理ext_vec
    let id = val.id || 0;
    // 处理
    return {
        id: id,
        group_id: val.group_id || 0,
        name: val.name || '',
        target_type: val.target_type || 0,
        target: val.target || '',
        logo_path: val.logo_path || '',
        create_time: val.create_time || getNow(),
        ext_vec: processExt(val, id, 0),
        start_vec: processAppStart(val.start_vec || [], id),
        remark: val.remark || '',
    }
}

export function processAppStart(start: AppStart[], app_id: number): AppStart[] {
    let start_vec: AppStart[] = [];
    for (let item of start) {
        start_vec.push({
            id: 0,
            app_id: app_id,
            name: item.name || '',
            remark: item.remark || '',
            ext_vec: processExt(item, app_id, 0),
            args: processArgs(app_id, item.args)
        });
    }
    return start_vec;
}

export function processArgs(app_id: number, args?: AppArgs[]): AppArgs[] {
    if (!args?.length) {
        return [];
    }
    let args_vec: AppArgs[] = [];
    for (let it of args) {
        let arg: any = it;
        args_vec.push({
            id: 0,
            start_id: 0,
            app_id: app_id,
            group_id: it.group_id || 0,
            ty: it.ty || ArgType.FIXED,
            name: it.name || '',
            default_value: it.default_value || '',
            remark: it.remark || '',
            config: it.config || '{}',
            multiple: arg.multiple === true ? 1 : 0,
            optional: arg.optional === true ? 1 : 0
        })
    }
    return args_vec;
}

export function processExt(val: any, app_id: number, start_id: number): AppExt[] {
    let ext_vec: AppExt[] = [];
    for (const key in val) {
        if (/ext\.*/.test(key)) {
            let it = val[key];
            let code = key.replace(/ext\.(.*)/, '$1');
            ext_vec.push({
                id: 0,
                app_id: app_id,
                start_id: start_id,
                ty: '',
                code: code,
                value: !it ? '' : JSON.stringify(it),
            })
        }
    }
    return ext_vec;
}

export function numberToBase64Img(data: number[]): string {
    let array = new Uint8Array(data);
    var len = array.byteLength;
    let str = '';
    for (let i = 0; i < len; i++) {
        str += String.fromCharCode( array[ i ] );
    }
    return `data:image/jpeg;base64,` + window.btoa(str);
}
