import { BaseDirectory, readTextFile, writeTextFile } from '@tauri-apps/api/fs'
import { DataStore } from '../../store/store';

/**
 * 更新数据文件
 * @param data 需要更新的内容
 * @returns  返回数据文件
 */
export async function updateDataFile(data: DataStore) {
    return writeTextFile('config/data.json', JSON.stringify(data, null, 4), { dir: BaseDirectory.App });
}
/**
 * 读取配置文件
 * @returns 
 */
export async function readDataFile(): Promise<DataStore> {
    return readTextFile('config/data.json', { dir: BaseDirectory.App }).then(content => {
        return JSON.parse(content) as DataStore;
    }).catch(reason => {
        const emptyDataStore: DataStore = {
            groups: [],
            settings: {}
        }
        updateDataFile(emptyDataStore);
        return emptyDataStore;
    });
}