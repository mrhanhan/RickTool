import {message} from "antd";
import {Code, CodeOperate} from "../Code";
import {InterpreterHandlerContext} from "../base";


export class AsmError {
    private _type: string;
    private _message: string;

    get type(): string {
        return this._type;
    }

    get message(): string {
        return this._message;
    }


    constructor(type: string, message: string) {
        this._type = type;
        this._message = message;
    }
}

export class GrammaticalError extends AsmError{

    private _code: Code;
    private _operate: CodeOperate;


    get code(): Code {
        return this._code;
    }

    get operate(): CodeOperate {
        return this._operate;
    }


    constructor(message: string, context: InterpreterHandlerContext) {
        super('语法错误', message);
        this._code = context.code;
        this._operate = context.operate;
    }
}
