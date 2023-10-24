import {MemoryUnit} from "./base";

export default class Register {

    /**
     * 通用寄存器
     */
    private _eax: MemoryUnit;
    private _ax: MemoryUnit;
    private _ah: MemoryUnit;
    private _al: MemoryUnit;
    private _ebx: MemoryUnit;
    private _bx: MemoryUnit;
    private _bh: MemoryUnit;
    private _bl: MemoryUnit;
    private _ecx: MemoryUnit;
    private _cx: MemoryUnit;
    private _ch: MemoryUnit;
    private _cl: MemoryUnit;
    private _edx: MemoryUnit;
    private _dx: MemoryUnit;
    private _dh: MemoryUnit;
    private _dl: MemoryUnit;
    /**
     * 浮点数寄存器
     */

    private _xmm0: MemoryUnit;
    private _xmm1: MemoryUnit;
    private _xmm2: MemoryUnit;
    private _xmm3: MemoryUnit;
    private _xmm4: MemoryUnit;
    private _xmm5: MemoryUnit;
    private _xmm6: MemoryUnit;
    private _xmm7: MemoryUnit;
    private _xmm8: MemoryUnit;
    private _xmm9: MemoryUnit;
    private _xmm10: MemoryUnit;
    private _xmm11: MemoryUnit;
    private _xmm12: MemoryUnit;
    private _xmm13: MemoryUnit;
    private _xmm14: MemoryUnit;
    private _xmm15: MemoryUnit;
    /**
     * 栈顶地址  段寄存器
     * @private
     */
    private _esp: MemoryUnit;
    private _sp: MemoryUnit;
    /**
     * 栈底地址 段寄存器
     * @private
     */
    private _ebp: MemoryUnit;
    private _bp: MemoryUnit;
    /**
     * 寄存器通常用于指向源数据的位置，即在数据传输和复制操作中，指示数据应该被复制的源。
     * @private
     */
    private _esi: MemoryUnit;
    private _si: MemoryUnit;
    /**
     * 寄存器通常用于指向目标位置，即在数据传输和复制操作中，用于指示数据应该被复制到的目的地。
     * @private
     */
    private _edi: MemoryUnit;
    private _di: MemoryUnit;
    /**
     * 代码段寄存器
     * @private
     */
    private _cs: MemoryUnit;
    /**
     * 存储数据段的基址
     * @private
     */
    private _ds: MemoryUnit;
    /**
     * 存储栈段的基址
     * @private
     */
    private _ss: MemoryUnit;
    /**
     * 通常用于额外的数据段
     * @private
     */
    private _es: MemoryUnit;
    /**
     * 标记控制
     * @private
     */
    private _eFlags: MemoryUnit;
    private _flags: MemoryUnit;
    /**
     * ip 寄存器
     * @private
     */
    private _eip: MemoryUnit;
    private _ip: MemoryUnit;


    get eax(): MemoryUnit {
        return this._eax;
    }

    get ax(): MemoryUnit {
        return this._ax;
    }

    get ah(): MemoryUnit {
        return this._ah;
    }

    get al(): MemoryUnit {
        return this._al;
    }

    get ebx(): MemoryUnit {
        return this._ebx;
    }

    get bx(): MemoryUnit {
        return this._bx;
    }

    get bh(): MemoryUnit {
        return this._bh;
    }

    get bl(): MemoryUnit {
        return this._bl;
    }

    get ecx(): MemoryUnit {
        return this._ecx;
    }

    get cx(): MemoryUnit {
        return this._cx;
    }

    get ch(): MemoryUnit {
        return this._ch;
    }

    get cl(): MemoryUnit {
        return this._cl;
    }

    get edx(): MemoryUnit {
        return this._edx;
    }

    get dx(): MemoryUnit {
        return this._dx;
    }

    get dh(): MemoryUnit {
        return this._dh;
    }

    get dl(): MemoryUnit {
        return this._dl;
    }

    get esp(): MemoryUnit {
        return this._esp;
    }

    get sp(): MemoryUnit {
        return this._sp;
    }

    get ebp(): MemoryUnit {
        return this._ebp;
    }

    get bp(): MemoryUnit {
        return this._bp;
    }

    get esi(): MemoryUnit {
        return this._esi;
    }

    get si(): MemoryUnit {
        return this._si;
    }

    get edi(): MemoryUnit {
        return this._edi;
    }

    get di(): MemoryUnit {
        return this._di;
    }

    get cs(): MemoryUnit {
        return this._cs;
    }

    get ds(): MemoryUnit {
        return this._ds;
    }

    get ss(): MemoryUnit {
        return this._ss;
    }

    get es(): MemoryUnit {
        return this._es;
    }

    get eFlags(): MemoryUnit {
        return this._eFlags;
    }

    get flags(): MemoryUnit {
        return this._flags;
    }


    get eip(): MemoryUnit {
        return this._eip;
    }

    get ip(): MemoryUnit {
        return this._ip;
    }


    get xmm0(): MemoryUnit {
        return this._xmm0;
    }

    get xmm1(): MemoryUnit {
        return this._xmm1;
    }

    get xmm2(): MemoryUnit {
        return this._xmm2;
    }

    get xmm3(): MemoryUnit {
        return this._xmm3;
    }

    get xmm4(): MemoryUnit {
        return this._xmm4;
    }

    get xmm5(): MemoryUnit {
        return this._xmm5;
    }

    get xmm6(): MemoryUnit {
        return this._xmm6;
    }

    get xmm7(): MemoryUnit {
        return this._xmm7;
    }

    get xmm8(): MemoryUnit {
        return this._xmm8;
    }

    get xmm9(): MemoryUnit {
        return this._xmm9;
    }

    get xmm10(): MemoryUnit {
        return this._xmm10;
    }

    get xmm11(): MemoryUnit {
        return this._xmm11;
    }

    get xmm12(): MemoryUnit {
        return this._xmm12;
    }

    get xmm13(): MemoryUnit {
        return this._xmm13;
    }

    get xmm14(): MemoryUnit {
        return this._xmm14;
    }

    get xmm15(): MemoryUnit {
        return this._xmm15;
    }

    public constructor() {
        this._eax = new MemoryUnit(32);
        this._ax = this._eax.createPartition(0, 16);
        this._al = this._eax.createPartition(0, 8);
        this._ah = this._eax.createPartition(8, 8);

        this._ebx = new MemoryUnit(32);
        this._bx = this._eax.createPartition(0, 16);
        this._bl = this._eax.createPartition(0, 8);
        this._bh = this._eax.createPartition(8, 8);

        this._ecx = new MemoryUnit(32);
        this._cx = this._eax.createPartition(0, 16);
        this._cl = this._eax.createPartition(0, 8);
        this._ch = this._eax.createPartition(8, 8);

        this._edx = new MemoryUnit(32);
        this._dx = this._eax.createPartition(0, 16);
        this._dl = this._eax.createPartition(0, 8);
        this._dh = this._eax.createPartition(8, 8);

        this._xmm0 = new MemoryUnit(32);
        this._xmm1 = new MemoryUnit(32);
        this._xmm2 = new MemoryUnit(32);
        this._xmm3 = new MemoryUnit(32);
        this._xmm4 = new MemoryUnit(32);
        this._xmm5 = new MemoryUnit(32);
        this._xmm6 = new MemoryUnit(32);
        this._xmm7 = new MemoryUnit(32);
        this._xmm8 = new MemoryUnit(32);
        this._xmm9 = new MemoryUnit(32);
        this._xmm10 = new MemoryUnit(32);
        this._xmm11 = new MemoryUnit(32);
        this._xmm12 = new MemoryUnit(32);
        this._xmm13 = new MemoryUnit(32);
        this._xmm14 = new MemoryUnit(32);
        this._xmm15 = new MemoryUnit(32);

        this._ebp = new MemoryUnit(32);
        this._bp = this._ebp.createPartition(0, 16);
        this._esp = new MemoryUnit(32);
        this._sp = this._esp.createPartition(0, 16);

        this._esi = new MemoryUnit(32);
        this._si = this._esi.createPartition(0, 16);
        this._edi = new MemoryUnit(32);
        this._di = this._edi.createPartition(0, 16);
        this._eip = new MemoryUnit(32);
        this._ip = this._eip.createPartition(0, 16);

        this._cs = new MemoryUnit(32);
        this._ds = new MemoryUnit(32);
        this._es = new MemoryUnit(32);
        this._ss = new MemoryUnit(32);
        this._eFlags = new MemoryUnit(32);
        this._flags = this._eFlags.createPartition(0, 16);
    }

    public get(name: string): MemoryUnit {
        // @ts-ignore
        return this[name];
    }
}
