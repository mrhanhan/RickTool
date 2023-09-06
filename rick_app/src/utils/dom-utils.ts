import {MouseEvent} from "react";


export function calculatePoint(element: HTMLElement): {x: number, y: number} {
    let rect = element.getBoundingClientRect();
    let x = rect.x;
    let y = rect.y;
    return {x, y};
}
