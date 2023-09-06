export enum ArgType {
    /**
     * 固定参数类型
     */
    FIXED = 1,
    /**
     * 文件参数类型
     */
    FILE = 2,
    /**
     * 输入参数类型
     */
    INPUT = 3,
    /**
     * 多选参数类型
     */
    SELECT = 4,
}

export interface ArgInputValue {
    id: number
    /**
     * 参数类型
     */
    ty: ArgType;
    /**
     * 参数名称
     */
    name: string;
    /**
     * 默认值
     */
    default_value?: string;
    /**
     * 配置信息 json 字符串
     */
    config?: string;
    /**
     * 是否可以添加多个
     */
    multiple?: number;
    /**
     * 参数是否可选
     */
    optional?: number;
    /**
     * 参数备注
     */
    remark?: string;
};


export const ArgTypeColorMap: Record<number|ArgType, string> = {
    1: 'blue',
    2: 'green',
    3: 'orange',
    4: 'pink',
};
export const ArgTypeNameMap: Record<number|ArgType, string> = {
    1: '固定参数',
    2: '文件参数',
    3: '输入参数',
    4: '多选参数',
};
