
export type CodeOperateArg = {
    /**
     * 参数值
     */
    value: number | string,
    /**
     * 值类型          value 取值为
     * ADDRESS: 地址  number: 内存地址 、表达式 : 寄存器 + - 偏移量
     * VALUE: 值      number: 值
     * REGISTER: 寄存器 value: 寄存器名称
     */
    type: 'ADDRESS' | 'VALUE' | 'REGISTER',
    /**
     * 32 位 16 位 8 位
     */
    bit: 32 | 16 | 8
};

export type CodeOperateInfo = {
    /**
     * 行号
     */
    lineNumber: number
};
export type CodeOperate = {
    /**
     * 代码地址
     */
    address: number;
    /**
     * 操作
     */
    operate: string;
    /**
     * 参数
     */
    args: CodeOperateArg[],
    info: CodeOperateInfo
};

export type CodeReference = {
    /**
     * 引用名称
     */
    name: string
    /**
     * 应用的类型 变量，还是Label
     */
    type: 'VARIABLE' | 'LABEL',
    /**
     * 引用值
     */
    value: string| number
};

export class Code {
    /**
     * 引用表
     * @private
     */
    private _referenceTable: Record<string, CodeReference> = {};
    /**
     * 操作
     * @private
     */
    private _operates: CodeOperate[] = [];


    get referenceTable(): Record<string, CodeReference> {
        return this._referenceTable;
    }

    get operates(): CodeOperate[] {
        return this._operates;
    }

    constructor() {
    }
}
