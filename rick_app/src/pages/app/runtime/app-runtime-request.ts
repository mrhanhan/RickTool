import {call} from "../../../utils/invoke";
import {Result} from "../../../model";

export interface AppRuntimeItem {
    id: number;
    app_runtime_id: number;
    code: string;
    value: string;
}

export type AppRuntimeItemForm = Partial<AppRuntimeItem>;
export type AppRuntimeForm = {items?: AppRuntimeItemForm[]} & Partial<AppRuntime> ;

export interface AppRuntime {
    id: number;
    name: string;
    description: string;
    include_system: number | boolean;
    items?: AppRuntimeItem[];
}


export interface AppRuntimeItemInputProps {
    value?: AppRuntimeItemForm[],
    onChange?: (value: AppRuntimeItemForm[]) => void
}

export async function listAppRuntime() {
    return call<Result<AppRuntime[]>>('/app/runtime/list');
}

export async function saveAppRuntime(model: AppRuntimeForm) {
    model.id = 0;
    return call<Result<AppRuntime>>('/app/runtime/save', processFormVal(model));
}
export async function updateAppRuntime(model: AppRuntimeForm, id: number) {
    model.id = id;
    return call<Result<AppRuntime>>('/app/runtime/update', processFormVal(model));
}
export async function deleteAppRuntime(id: number) {
    return call<Result<number>>('/app/runtime/delete', {id: id});
}

export async function detailAppRuntime(id: number) {
    return call<Result<AppRuntime>>('/app/runtime/detail', {id: id});
}


function processFormVal(model: AppRuntimeForm): AppRuntime {
    model.include_system = !model.include_system ? 0 : 1;
    console.log(model.items);
    if (model.items?.length) {
        model.items = model.items.filter(it => !!it.code && !!it.value)
            .map(it=> {
            it.id = 0;
            it.app_runtime_id = model.id;
            return it as AppRuntimeItem;
        });
        console.log(model);
    } else {
        model.items = [];
    }
    return model as AppRuntime;
}
