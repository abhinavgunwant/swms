import React, { ReactNode } from 'react';

type DefaultFunction = () => void;

export interface ThumbnailAction {
    label: string,
    icon: ReactNode,
    show: boolean,
    action: React.MouseEventHandler<HTMLDivElement>
        | React.MouseEventHandler<HTMLButtonElement>
        | React.MouseEventHandler<HTMLElement> | DefaultFunction | ((e:any) => void),
    text?: string,
    tooltip?: string,
    showTooltip?: boolean,
    onHideTooltip?: () => void,
}

/**
 * Prop type for `Thumbnail` and `ImageListItem`.
 */
//export default interface ThumbnailExtendedProps extends ThumbnailItemModel {
export default interface ThumbnailExtendedProps {
    id: number,     // same as project id
    name: string,   // same as project name
    key: number,
    subtitle?: string,
    thumbnailLocation?: string,
    type?: "IMAGE" | "FOLDER" | "PROJECT",
    onClick?: React.MouseEventHandler<HTMLDivElement> | undefined;
    actions?: ThumbnailAction[],
    isImage?: boolean;
    selected?: boolean,
    slug?: string,
};

