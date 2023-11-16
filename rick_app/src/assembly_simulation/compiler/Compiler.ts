import Program from "../Program";
import {Code, CodeOperate} from "../Code";
import {MemoryUnit} from "../base";
import Interpreter from "../Interpreter";


type Word = {
    type: 'CODE' | 'REMARK' | 'LABEL'
    content: string,
}

/**
 * 需要识别指令，标签，地址，
 */
export class Compiler {

    /**
     * 代码内容
     * @private
     */
    private _content: string;

    private _readIndex = 0;
    private _lineNumber = 1;
    private _interpreter: Interpreter;
    constructor(content: string, interpreter: Interpreter) {
        this._content = content;
        this._interpreter = interpreter;
    }

    public compile(): Program {
        const code = new Code();
        do {
            const world = this._readWord();
        }while (true);
        return new Program(code, new MemoryUnit(32));
    }

    private _readWord(): Word {
        // 读取内容
        const {_content, _interpreter} = this;
        const go = (offset=1) => this._readIndex += offset;
        const back = (offset=1) => this._readIndex -= offset;
        const index = (offset=0) => this._readIndex + offset;
        const line = () => this._lineNumber;
        const nextLine = () => this._lineNumber += 1;
        let word = '';
        let type: Word['type'] = 'CODE';
        // do {
        //     const code = _content.charAt(index());
        //     // 判断第一个字符是否是 ; 是注释
        //     // 判断第一个字符串是否是支持的代码操作
        //     // 否则是
        //     if (word === '') {
        //     }
        // }while (false);
        return {content: '', type: 'REMARK'} as Word;
    }

}
