import React from 'react';

import ThumbnailItemModel from "./ThumbnailItemModel";

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
    isImage?: boolean;
};
