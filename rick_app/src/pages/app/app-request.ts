import {App, AppGroup} from "./app-model";
import {call} from "../../utils/invoke";
import {Result} from "../../model";


export async function listAppGroup() {
    return call<Result<AppGroup[]>>('/app/group/list');
}

export async function saveAppGroup(name: string) {
    return call<Result<AppGroup>>('/app/group/save', {id: 0, name, icon: ''});
}

export async function updateAppGroup(id: number, name: string) {
    return call<Result<AppGroup>>('/app/group/update', {id, name, icon: ''});
}

export async function delAppGroup(id: number) {
    return call<Result<AppGroup>>('/app/group/delete', {id});
}

export async function listApp(queryModel: {group_id?: number, keyword?: string}) {
    return call<Result<App[]>>('/app/list', queryModel);
}

export async function saveApp(model: App) {
    return call<Result<App>>('/app/save', model);
}

