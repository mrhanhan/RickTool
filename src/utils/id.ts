
const MAP = 'abcdefghigklmnovwzyx0987654321';
let count = 0;

/**
 * 
 * @returns 返回生成的ID
 */
export function genId(): string {
    let prefix = new Date().getTime().toString(16).toUpperCase();
    let str = count.toString(16).toUpperCase();
    for (let i = str.length; i < 4; i++) {
        str = '0' + str;
    }
    count ++;
    return prefix + str;
};