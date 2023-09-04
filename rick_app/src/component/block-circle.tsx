import {CSSProperties, ReactNode} from "react";

import "./text-circle.less"

export default function BlockCircle(props: {children: ReactNode|ReactNode[], color?: string, style?: CSSProperties}) {
    let style: any = {...(props.style || {})};
    if (props.color) {
        style['--border-color'] = props.color;
    }
    return <div className={"block-circle"} style={style as CSSProperties}>
        {props.children}
    </div>
}
