import {CodeOperateArg} from "./Code";
import {InterpreterHandlerContext, MemoryUnit} from "./base";
import {GrammaticalError} from "./exception/Exception";


export function calculateMemoryUnit(arg: CodeOperateArg, context: InterpreterHandlerContext): MemoryUnit {
    return context.memory;
}

export function writeValue(arg: CodeOperateArg, context: InterpreterHandlerContext, value: number) {
    if (arg.type === 'VALUE') {
        throw new GrammaticalError(`无法写入`, context);
    }
    if (arg.type === 'REGISTER') {
        context.register.get(arg.value as string).setBitN(0, arg.bit, value);
    }
    if (arg.type === 'ADDRESS') {
        const address = _calculateAddress(arg, context);
        context.memory.setBitN(address, arg.bit, value);
    }

    throw new GrammaticalError(`写入 ${arg.value} 参数错误，未知类型 ${arg.type}`, context);
}

/**
 * 计算内存地址 只针对于 arg 类型 = ADDRESS
 * @param arg 参数
 * @param context
 */
function _calculateAddress(arg: CodeOperateArg, context: InterpreterHandlerContext) {

    // 计算表达式
    // eax +- number
    let mode = false;
    const expression: string = arg.value as string;
    if (expression.indexOf('+') > 0) {
        mode = true;
    }
    const values = expression.split(/\+-/);
    // 判断是否存在两个参数
    if (values.length < 1) {
        throw new GrammaticalError(`Arg ${arg.value} 语法错误`, context);
    }
    const registerName = values[0];
    // 寄存器名称
    let address = 0;
    // 是否是内存地址
    if (isNumber(registerName)) {
        address = toNumber(registerName);
    } else {
        address = context.register.get(registerName).readBitN(0, arg.bit);
    }
    // 判断是否只有几个地址
    if (values.length === 1) {
        return address;
    }
    // 判断是否只有第二个子
    if (!isNumber(values[1])) {
        throw new GrammaticalError(`${arg.value} 参数表达式错误: 寄存器 +- 数字`, context);
    }
    return address + ((mode ? -1 : 1) * toNumber(values[1]));
}

export function calculateValue(arg: CodeOperateArg, context: InterpreterHandlerContext): number {
    if (arg.type === 'VALUE') {
        return argValueToNumber(arg);
    }
    // 判断是否是内存地址
    if (arg.type === 'ADDRESS') {
        const address = _calculateAddress(arg, context);
        return context.memory.readBitN(address, arg.bit);
    }
    // 判断是否是寄存器
    if (arg.type === 'REGISTER') {
        return context.register.get(arg.value as string).readBitN(0, arg.bit);
    }
    throw new GrammaticalError(`${arg.value} 参数错误，未知类型 ${arg.type}`, context);
}


export function argValueToNumber(arg: CodeOperateArg): number {
    if (typeof arg.value === 'string') {
        return parseInt(arg.value);
    } else {
        return arg.value;
    }
}

export function isNumber(value: string) {
    return /^\d+$/.test(value) || !isNaN(Number(value));
}

export function toNumber(value: string): number {
    return Number(value);
}
