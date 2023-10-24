import {Code} from "./Code";
import {MemoryUnit} from "./base";

/**
 * 程序
 * 程序包含了，数据区域，代码区域
 */
export default class Program {
    /**
     * 代码片段
     * @private
     */
    private _code: Code;
    /**
     * 内存数据
     * @private
     */
    private _memory: MemoryUnit;


    get code(): Code {
        return this._code;
    }

    get memory(): MemoryUnit {
        return this._memory;
    }

    constructor(code: Code, memory: MemoryUnit) {
        this._code = code;
        this._memory = memory;
    }
}
