import {InterpreterHandler, InterpreterHandlerContext} from "../base";
import {CodeOperate} from "../Code";
import {calculateValue, writeValue} from "../operate_utils";

/**
 * 语法：
 *
 * mov dest, src
 * mov eax, 10
 * mov eax, ebx
 * mov DWORD PTR [number|eax|ebx + x], ebx
 *
 * @param code
 * @param context
 */
export const movHandler: InterpreterHandler = (code: CodeOperate, context: InterpreterHandlerContext) => {
    const {interpreter} = context;
    const dest = code.args[0];
    const src = code.args[1];
    if (dest.type === 'VALUE') {
        throw 'mov 语法错误 line: ' + code.info.lineNumber;
    }
    // 计算得到原本的值
    let val: number = calculateValue(src, context);
    // 写入目的地
    writeValue(dest, context, val);
};
