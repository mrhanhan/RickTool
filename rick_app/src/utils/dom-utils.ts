import {MouseEvent} from "react";

export function calculateMousePointIndex(parent: HTMLElement, event: MouseEvent<HTMLElement>): number {
    let children = parent.children;
    let pointX = event.clientX;
    let pointY = event.clientY;
    let index = -1;
    for (let i = 0; i < children.length; i++) {
        let htmlElement = children.item(i) as HTMLElement;
        if (!htmlElement) {
            continue;
        }
        let rect = htmlElement.getBoundingClientRect();
        if (rect.x <= pointX && rect.x + rect.width >= pointX && rect.y <= pointY && rect.y + rect.height >= pointY) {
            return i;
        }
    }
    return index;
}

export function calculatePoint(element: HTMLElement): {x: number, y: number} {
    let rect = element.getBoundingClientRect();
    let x = rect.x;
    let y = rect.y;
    return {x, y};
}
