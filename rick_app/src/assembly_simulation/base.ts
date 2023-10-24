import {Code, CodeOperate} from "./Code";
import Interpreter from "./Interpreter";
import Register from "./Register";
import Program from "./Program";

/**
 * 内存单元
 */
export class MemoryUnit {
    /**
     * 二进制数据
     * @private 数据
     */
    private _data: number[];

    private _size: number;

    private _offset: number = 0;

    /**
     * 获取内存单元数据
     */
    public get data() {
        return this._data.slice(this._offset, this._offset + this._size);
    }

    public get size() {
        return this._size;
    }
    /**
     * 内存大小
     * @param size 内存大小 二进制的长度
     */
    public constructor(size: number ) {
        this._data = new Array(size).fill(0);
        this._size = size;
    }


    public createPartition(start: number, size: number): MemoryUnit {
        const unit = new MemoryUnit(size);
        unit._data = this._data;
        unit._offset = start;
        return unit;
    }

    /**
     * 设置Bit
     * @param pos 位子
     * @param bit bit
     */
    public setBit(pos: number, bit: number) {
        this._data[pos + this._offset] = bit;
    }

    /**
     * 设置Bit
     * @param pos 位子
     * @param word bit
     */
    public setBit2(pos: number, word: number) {
        this._setBitN(pos, word, 2);
    }

    public setBit4(pos: number, word: number) {
        this._setBitN(pos, word, 4);
    }

    public setBit8(pos: number, word: number) {
        this._setBitN(pos, word, 8);
    }

    public setBit16(pos: number, word: number) {
        this._setBitN(pos, word, 16);
    }

    public setBit32(pos: number, word: number) {
        this._setBitN(pos, word, 32);
    }
    private _setBitN(pos: number, word: number, bitCount: number) {
        let array = Number(word).toString(2).split("").map(bit => parseInt(bit));
        if (pos + bitCount > this.size) {
            throw 'OutMemory';
        }
        array = array.slice(Math.min(array.length - bitCount, 0));
        for (let i = 0; i < bitCount; i++) {
            this._data[pos + this._offset + i] = array[array.length - 1 - i] || 0;
        }
    }
    public setBitN(pos: number, value: number, bitCount: number) {
        this._setBitN(pos, value, bitCount);
    }

    private _readBitN(pos: number, bitCount: number): number {
        let num = 0;
        for (let i = 0; i < bitCount; i++) {
            num |= this._data[this._offset + pos];
            num <<= 1;
        }
        return num;
    }

    public readBitN(pos: number, bitCount: number): number {
        return this._readBitN(pos, bitCount);
    }

}

export type InterpreterHandlerContext = {
    /**
     * 解释器
     */
    interpreter: Interpreter,
    /**
     * 寄存器
     */
    register: Register,
    /**
     * 程序
     */
    program: Program,
    /**
     * 内存地址
     */
    memory: MemoryUnit,
    /**
     * 内存地址
     */
    code: Code,
    /**
     * 操作
     */
    operate: CodeOperate
};

export interface InterpreterHandler {
    (operate: CodeOperate, context: InterpreterHandlerContext): void
}
