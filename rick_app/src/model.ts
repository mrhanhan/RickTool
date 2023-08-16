import {MessageInstance} from "antd/es/message/interface";
import {message} from "antd";


export interface Result<T> {
    code: number;
    data: T;
    message: string;
}

export async function commonProcess<T>(data: Promise<Result<T>>): Promise<T> {
    return data.then(result => {
        if (result.code === 200) {
            return result.data;
        }
        throw result;
    });
}

export function errorMessage(api: MessageInstance) {
    return (reason?: string|Result<any>) => {
        if (!reason) {
            api.error('操作失败').then();
            return;
        }
        if (typeof reason === 'string') {
            api.error(reason).then();
        } else {
            api.error(reason.message).then();
        }
    }
}
