import {ReactNode} from "react";
import {List} from "antd";
import {useContextMenuHolder} from "./context-menu";

declare interface ContextMenuItemProps {
    children: ReactNode|ReactNode[],
    bind?: any
}

export default function ContextMenuItem(props: ContextMenuItemProps) {
    let holder = useContextMenuHolder();
    return <div onContextMenu={(e) => {
        holder?.onContextMenu(e, props.bind);
        e.stopPropagation();
        e.preventDefault();
    }}>
        {props.children}
    </div>
}
