import React, { ReactNode } from 'react';

type DefaultFunction = () => void;

interface ThumbnailAction {
    label: string,
    icon: ReactNode,
    show: boolean,
    action: React.MouseEventHandler<HTMLDivElement> | DefaultFunction,
}

/**
 * Prop type for `Thumbnail` and `ImageListItem`.
 */
//export default interface ThumbnailExtendedProps extends ThumbnailItemModel {
export default interface ThumbnailExtendedProps {
    id: number,     // same as project id
    name: string,   // same as project name
    thumbnailLocation: string,
    key: number,
    onClick?: React.MouseEventHandler<HTMLDivElement> | undefined;
    actions?: ThumbnailAction[],
    isImage?: boolean;
    selected?: boolean,
};

