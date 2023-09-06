export interface AppGroup {
    id: number;
    name: string;
    icon: string;
}

export interface App {
    id: number;
    group_id: number;
    name: string;
    target_type: number;
    target: string;
    logo_path: string;
    remark: string;
    create_time: number;
    logo?: number[];
    ext_vec?: AppExt[];
    start_vec?: AppStart[];
}

export interface AppStart {
    id: number;
    app_id: number;
    remark: string;
    name: string;
    ext_vec?: AppExt[];
    args?: AppArgs[];
}

export interface AppArgs {
    id: number;
    app_id: number;
    start_id: number;
    group_id: number;
    ty: number;
    name: string;
    default_value: string;
    config: string;
    multiple: number;
    optional: number;
    remark: string;
}

export interface AppExt {
    id: number;
    app_id: number;
    start_id: number;
    ty: string;
    code: string;
    value: string;
}
