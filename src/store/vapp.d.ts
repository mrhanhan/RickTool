import { EnvironmentItem } from "../service/environment";

/**
 * 虚拟APP 分组
 */
export declare interface VAppGroup extends Identification {
    /**
     * 分组图标
     */
    icon?: string;
    /**
     * 虚拟分组名称
     */
    name?: string;
    /**
     * 分组的虚拟APP
     */
    app: VApp[]
}

export declare type InputType = 'INSTRUCTION' | 'NORMAL' | 'FILE';
/**
 * 虚拟App 参数
 */
export declare interface VAppArg extends Identification {
    /**
     * 参数名称
     */
    name?: string;
    /**
     * 操作参数 例如 -i 可控
     */
    opt?: string;
    /**
     * 参数值
     */
    value?: string;
    /**
     * 输入类型 
     * INPUT     启动 vApp 时需要用户输入
     * NORMAL    无需做任何设置
     * SELECT    用户选择几个内容
     */
    inputType?: InputType;
    /**
     * 可选择的数据
     */
    optional?: string[];
}

/**
 * 虚拟APP
 */
export declare interface VApp extends Identification {
    /**
     * VApp 名称
     */
    name: string;
    /**
     * 图标 图片
     */
    icon?: string;
    /**
     * 运行目录
     */
    dir?: string;
    /**
     * 启动的目标
     */
    target: string;
    /**
     * 启动目标类型
     * URL      打开URL
     * APP      可执行程序
     * SHELL    执行DOS 或者 Shell命令
     * SYSTEM   系统应用
     */
    targetType: 'URL' | 'APP' | 'SHELL' | 'SYSTEM';
    /**
     * vApp 启动参数
     */
    args?: VAppArg[];
    /**
     * VApp 运行环境
     */
    environment?: EnvironmentItem[];

    /**
     * 启动这个 VApp 的快捷键
     */
    shortcutKey?: string;
    /**
     * 运行时是否显示结果窗口
     */
    shell?: boolean;
    /**
     * 管理员模式运行
     */
    sudo?: boolean;
}