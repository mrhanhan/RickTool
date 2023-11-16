import Register from "./Register";
import {InterpreterHandler, MemoryUnit} from "./base";
import {movHandler} from "./handler/mov";

/**
 * 汇编解释器
 */
export default class Interpreter {
    /**
     * 寄存器
     * @private
     */
    private _register: Register;
    /**
     * 内存单元
     * @private
     */
    private _memory: MemoryUnit;
    /**
     * 运行状态
     * @private
     */
    private _status: boolean;
    /**
     * 处理器
     * @private
     */
    private _handlerMap: Record<string, InterpreterHandler> = {};

    get register(): Register {
        return this._register;
    }

    set register(value: Register) {
        this._register = value;
    }

    get memory(): MemoryUnit {
        return this._memory;
    }

    set memory(value: MemoryUnit) {
        this._memory = value;
    }

    get status(): boolean {
        return this._status;
    }

    set status(value: boolean) {
        this._status = value;
    }

    public constructor(options: {memory: number}) {
        this._register = new Register();
        this._memory = new MemoryUnit(options.memory);
        this._status = false;
        this._addDefaultHandler();
    }

    public addHandler(name: string, handler: InterpreterHandler) {
        this._handlerMap[name] = handler;
    }


    private _addDefaultHandler() {
        this.addHandler('mov', movHandler);
        this.addHandler('push', movHandler);
        this.addHandler('pull', movHandler);
        this.addHandler('je', movHandler);
        this.addHandler('jmp', movHandler);
        this.addHandler('lea', movHandler);
        this.addHandler('call', movHandler);
        this.addHandler('ret', movHandler);
        this.addHandler('nop', movHandler);
        this.addHandler('xor', movHandler);
        this.addHandler('add', movHandler);
        this.addHandler('sub', movHandler);
        this.addHandler('jne', movHandler);
    }
}
