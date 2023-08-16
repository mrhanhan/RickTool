import {call} from "./invoke";
import {Result} from "../model";


export interface FileOpen {
    /**
     * 文件类型过滤
     */
    filter?: {name: string, extensions?: string[]}[],
    /**
     * 对话框标题
     */
    title?: string,
    /**
     * 默认文件名称
     */
    default_name?: string,
    /**
     * 默认打开路径
     */
    default_dir?: string,
    /**
     * 是否支持多选
     */
    multiple?: boolean,
    /**
     * 是否需要文件内容信息
     */
    need_content?: boolean
}

export interface FileOpenResult {
    /**
     * 选择路径
     */
    path: string,
    /**
     * 文件内容
     */
    content?: number[],
    /**
     * 内容状态
     */
    content_status?: boolean,
    /**
     * 内容读取失败原因
     */
    fail_reason?: string
}

export async function fileReadDialogOpen(options: FileOpen = {}) {
    options = {need_content: true, multiple: false, ...options};
    return call<Result<FileOpenResult[]>>('/common/file/read/open', options);
}

export async function fileSaveDialogOpen(options: FileOpen = {}) {
    options = {need_content: false, multiple: false, ...options};
    return call<Result<FileOpenResult[]>>('/common/file/save/open', options);
}

export async function dirDialogOpen(options: FileOpen = {}) {
    options = {need_content: false, multiple: false, ...options};
    return call<Result<FileOpenResult[]>>('/common/dir/open', options);
}
