import { useState, useEffect } from 'react';
import LinkModel from '../../../models/LinkModel';

import WorkspaceTopRow from "../WorkspaceTopRow";

interface ViewImageProps {
    imageId?: string;
}

const ViewImage = (props: ViewImageProps) => {
    const [breadcrumbLinks, setBreadcrumbLinks ] = useState<Array<LinkModel | string>>(['Workspace']);

    useEffect(() => {
        //// TODO: query backend and get the full details of the image from
        //// the image id passed into the props

        setBreadcrumbLinks([
            {
                text: 'Workspace',
                to: '/workspace',
            },
            {
                text: 'Product Images',
                to: '/workspace/tree/product-images',
            },
            'Scrumtools.io Logo!',
        ]);
    }, []);

    return <div className="page page--view-image">
        <WorkspaceTopRow links={ breadcrumbLinks } />
    </div>
}

export default ViewImage;
