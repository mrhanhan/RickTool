
export function isFunction(obj: any): boolean {
    return typeof obj === 'function';
}

export function isArray(obj: any): boolean {
    return obj instanceof Array;
}