import {CSSProperties, ReactNode} from "react";

import "./text-circle.less"

export default function TextCircle(props: {children: ReactNode[], color?: string}) {
    let style: any = {};
    if (props.color) {
        style['--border-color'] = props.color;
    }
    return <span className={"text-circle"} style={style as CSSProperties}>
        {props.children}
    </span>
}
