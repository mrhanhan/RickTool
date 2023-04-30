import { EnvironmentItem } from "../store/store"

export declare interface EnvironmentItem {
    id: string, key: string, value: string
}

export declare interface EnvironmentGroup {
    /**
     * ID 类型
     */
    id?: string;
    /**
     * 环境名称
     */
    name?: string;
    /**
     * 包含系统变量
     */
    includeSystem?: boolean;
    /**
     * 描述信息
     */
    description?: string;
    /**
     * 环境变量
     */
    envs?: EnvironmentItem[];
}
