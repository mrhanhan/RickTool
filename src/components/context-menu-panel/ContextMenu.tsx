import React, { useEffect, useState } from "react";
import { ContextMenuItem } from "./ContextMenuItem";
import './ContextMenu.css';

type ContextMenuChildren = ReturnType<typeof ContextMenuItem> | ReturnType<typeof ContextMenuItem>[];
/**
 * ContextMenu
 */
declare interface ContextMenuProps {

    id: string;
    /**
     * 
     */
    children: ContextMenuChildren | ((data: any) => ContextMenuChildren);
    /**
     * 最大宽度
     */
    width?: number;
}
/**
 * 显示ContextMenu
 * @param id       需要显示的ContextMenu Id
 * @param point    显示的位置
 * @param data     触发的数据
 */
export function showContextMenu(id: string, point: { x: number, y: number }, data: any) {
    if (CONTEXT_MENU_MAP[id]) {
        CONTEXT_MENU_MAP[id].show(point, data);
    }
}
/**
 * 隐藏ContextMenu
 * @param id       需要显示的ContextMenu Id
 */
export function hideContextMenu(id: string) {
    if (CONTEXT_MENU_MAP[id]) {
        CONTEXT_MENU_MAP[id].hide();
    }
}

export function hideAll() {
    for (let context of Object.values(CONTEXT_MENU_MAP)) {
        context.hide();
    }
}

const CONTEXT_MENU_MAP: Record<string, { show: (point: { x: number, y: number }, data: any) => void, hide: () => void }> = {};

export const ContextMenuDataContext = React.createContext(null as any);
window.addEventListener('click', () => {
    hideAll();
});
/**
 * 清除
 */
window.oncontextmenu = (e: Event) => {
    e.preventDefault();
    hideAll();
};

export function ContextMenu(props: ContextMenuProps) {
    const [data, setData] = useState({ data: null, contextMenuId: props.id });
    const [visible, setVisible] = useState(false);
    const [point, setPoint] = useState({ x: 0, y: 0 });
    CONTEXT_MENU_MAP[props.id] = {
        show: (point: { x: number, y: number }, d: any) => {
            setPoint(point);
            setData({ data: d, contextMenuId: props.id });
            setVisible(true);
            // console.log('show');
        },
        hide: () => {
            setData({ data: null, contextMenuId: props.id });
            setVisible(false);
            // console.trace('hide');
        }
    };
    useEffect(() => () => {
        delete CONTEXT_MENU_MAP[props.id];
    }, []);
    const children = typeof props.children === 'function' ? props.children(data) : props.children;
    let styles: Record<string, any> = {};
    if (props.width) {
        styles['width'] = `${props.width}px`;
        styles['maxWidth'] = `${props.width}px`;
    }
    // console.log('render ContextMenu', visible);
    return <ContextMenuDataContext.Provider value={data}>
        {visible && <>
            {/* TODO 遮罩 用来实现隐藏 */}
            <div style={{ ...styles, top: `${point.y}px`, left: `${point.x}px`}} className="context-menu_wrapper">
                {children}
            </div>
        </>}
    </ContextMenuDataContext.Provider>
}