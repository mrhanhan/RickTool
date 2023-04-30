import { BaseDirectory, appConfigDir } from '@tauri-apps/api/path';
import { createDir} from '@tauri-apps/api/fs';

/**
 * 初始化App 数据环境
 */
export async function initAppDataEnv() {
    console.log(await appConfigDir());
    // 创建配置目录
    createDir('config', { dir: BaseDirectory.App, recursive: true }).then(() => {
        console.log('config 目录不存在');
    }).catch(() => {
        console.log('config 目录已存在');
    });
    // 读取环境文件
    
}

/**
 * 初始化App
 */
export function initApp() {
    // Promise.all([initAppDataEnv()]).then(() => {

    // });
}