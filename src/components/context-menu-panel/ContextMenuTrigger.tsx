import React, { isValidElement } from "react";
import { showContextMenu } from "./ContextMenu";

declare interface ContextMenuTriggerProps {
    /**
     * 显示的ContextMenu
     */
    contextMenuId: string;
    /**
     * 触发的数据
     */
    data: any;
    /**
     * 子节点
     */
    children: React.ReactNode;
    /**
     * 点击事件触发
     */
    clickTrigger: boolean;
}

function cloneElement(clone: React.ReactNode, props: Record<string, any>): React.ReactNode {
    if (!isValidElement(clone)) {
        console.log('Not isValidElement', clone);
        return clone;
    } else {
        if (typeof clone.type !== 'string') {
            props = { contextMenu: props };
        }
    }
    return React.cloneElement(clone, { ...clone.props, ...props });
}

export function ContextMenuTrigger(props: ContextMenuTriggerProps) {
    const newProps: Record<string, any> = {

    };
    newProps.onContextMenu = (e: MouseEvent) => {
        e.preventDefault && e.preventDefault();
        e.stopPropagation && e.stopPropagation();
        console.log('ContextMenu');
        showContextMenu(props.contextMenuId, { x: e.clientX, y: e.clientY }, props.data);
    };
    if (props.clickTrigger) {
        newProps.onClick = (e: MouseEvent) => {
            e.preventDefault && e.preventDefault();
            e.stopPropagation && e.stopPropagation();
            console.log('');
            showContextMenu(props.contextMenuId, { x: e.clientX, y: e.clientY }, props.data);
        };
    }

    const NewChildren = cloneElement(props.children, newProps);
    return <div className="demo">{NewChildren}</div>;
}