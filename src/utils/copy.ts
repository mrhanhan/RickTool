import React, { HTMLAttributes, ReactHTMLElement } from "react";
import { isValidElement } from "react";


export function copy<T>(obj: T): T {
    return JSON.parse(JSON.stringify(obj));
}

export function of<R>(obj: any):R {
    return obj as R;
}

type AnyObject = Record<any, any>;

type RenderProps = undefined | AnyObject | ((originProps: AnyObject) => AnyObject | undefined);

export function cloneElement(clone: React.ReactNode, props?: RenderProps): React.ReactNode {
    if (!isValidElement(clone)) {
        console.log('Not isValidElement', clone);
        return clone;
    } else {
        if(typeof clone.type === 'string') {
            
        }
    }
    return React.cloneElement(clone, typeof props === 'function' ? props(clone.props) : props);
}