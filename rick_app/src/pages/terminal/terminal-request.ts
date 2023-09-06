import {call} from "../../utils/invoke";
import {Result} from "../../model";
import {TerminalModel} from "./terminal-model";

export async function listTerminal() {
    return call<Result<TerminalModel[]>>('/terminal/list', {});
}


export async function createTerminal(model: TerminalModel) {
    return call<Result<TerminalModel>>('/terminal/create', model);
}

export async function killTerminal(model: {id: string}) {
    return call<Result<any>>('/terminal/kill', model);
}

export async function readTerminal(model: {id: string, offset: number}) {
    return call<Result<number[]>>('/terminal/read', model);
}


export async function writeTerminal(model: {id: string, data: number[]}) {
    return call<Result<void>>('/terminal/write', model);
}

