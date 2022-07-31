import React from 'react';

import ThumbnailItemModel from "./ThumbnailItemModel";

/**
 * Prop type for `Thumbnail` and `ImageListItem`.
 */
export default interface ThumbnailExtendedProps extends ThumbnailItemModel {
    onClick?: React.MouseEventHandler<HTMLDivElement> | undefined;
    isImage?: boolean;
};
