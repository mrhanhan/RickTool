import {createContext, ReactNode, useContext, useState, MouseEvent, useRef, useEffect} from "react";

import './index.less';

declare interface ContextMenuProps {
    /**
     * 渲染数据
     */
    onRender?: (value: any) => ReactNode | ReactNode[],

    children: ReactNode | ReactNode[]
}

declare interface ContextMenuHolder {
    onContextMenu: (e: MouseEvent<HTMLDivElement>, value: any) => void;

    hide: () => void;
}

const ContextMenuContext = createContext<ContextMenuHolder | null>(null);

export function useContextMenuHolder(): ContextMenuHolder | null {
    return useContext(ContextMenuContext)
}

export default function ContextMenu(props: ContextMenuProps) {

    const containerRef = useRef<HTMLDivElement|null>(null);
    const [state, setState] = useState({x: 0, y: 0, visible: false, value: null});
    const onContextMenu = (e: MouseEvent<HTMLDivElement>, value: any) => {
        const {x, y} = containerRef.current?.getBoundingClientRect()!;
        setState({...state, x: e.pageX - x, y: e.pageY - y, value, visible: true});
    };

    useEffect(() => {
        document.addEventListener('click', onContextMenuClose);
        document.addEventListener('contextmenu', onContextMenuClose);
        return () => {
            document.removeEventListener('contextmenu', onContextMenuClose);
            document.removeEventListener('click', onContextMenuClose);
        };
    }, []);

    const onContextMenuClose = () => {
      setState({...state, visible: false});
    };
    return <div ref={containerRef} className={"context-menu-wrapper"} onContextMenu={onContextMenuClose}
                onClick={onContextMenuClose}>
        <ContextMenuContext.Provider value={{onContextMenu, hide: onContextMenuClose}}>
            {props.children}
        </ContextMenuContext.Provider>
        {state.visible && <div className={`context-menu-layout`} style={{top: `${state.y}px`, left: `${state.x}px`}}>
            {props.onRender?.(state.value)}
        </div>}
    </div>;
}
