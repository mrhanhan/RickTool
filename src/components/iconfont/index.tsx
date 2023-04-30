
import './index.css';

declare interface IconFontProps {
    /**
     * ICON 名称
     */
    icon: string;
    /**
     * 大小
     */
    size?: number;
}

export default function IconFont(props: IconFontProps) {

    return <svg className="FontIcon" aria-hidden="true">
        <use xlinkHref={`#icon-${props.icon}`}></use>
    </svg>
}