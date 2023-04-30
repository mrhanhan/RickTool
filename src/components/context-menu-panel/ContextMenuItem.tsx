
import { Col, Row } from "antd";
import { ContextMenuDataContext, hideContextMenu } from "./ContextMenu";
import './ContextMenuItem.css'
import React from "react";
declare interface ContextMenuItemProps {
    /**
     * 点击事件
     */
    onClick?: (data: any) => void | Promise<any>;
    children: React.ReactNode;
    icon?: React.ReactNode;
}

export function ContextMenuItem(props: ContextMenuItemProps) {
    return <ContextMenuDataContext.Consumer>
        {
            (e: { data: any, contextMenuId: string }) => {
                return <Row className="ContextMenuItem_wrapper" onClick={() => {
                    const p = props.onClick?.(e.data);
                    if (p) {
                        // 隐藏
                        p.finally(() => {
                            hideContextMenu(e.contextMenuId);
                        });
                    } else {
                        hideContextMenu(e.contextMenuId);
                    }
                }}>
                    {props.icon && <Col flex="24px">
                        {props.icon}
                    </Col>}
                    <Col flex="1 1 0">
                        {props.children}
                    </Col>
                </Row>
            }
        }
    </ContextMenuDataContext.Consumer>
}