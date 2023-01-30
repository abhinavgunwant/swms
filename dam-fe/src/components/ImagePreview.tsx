import { Fragment, useState, useEffect, useTransition } from 'react';
import { Backdrop, IconButton } from '@mui/material';
import { Image as ImageIcon, ImageNotSupported, ZoomIn, ZoomOut } from '@mui/icons-material';

import styled from '@emotion/styled';
import { styled as muiStyled } from '@mui/material/styles';

const StyledBackdrop = muiStyled(Backdrop)`
    display: flex;
    flex-direction: column;
`;

const ImageSection = styled.div`
    display: flex;
    justify-content: center;
    align-items: center;

    width: 100%;
    height: calc(100vh - 100px);
`;

const ControlSection = styled.div`
    display: flex;
    justify-content: center;
    align-items: center;

    width: 100%;
    height: 100px;
`;

interface ImagePreviewProps {
    imageId?: number,
    show?: boolean,
}

export const ImagePreview = (props: ImagePreviewProps) => {
    const [ opened, setOpened ] = useState<boolean>(props?.show || false);
    const [ imagePath, setImagePath ] = useState<string>('');
    const [ imageId, setImageId ] = useState<number>(props?.imageId || -1);
    const [ zoom, setZoom ] = useState<number>(100);

    const [ _, startTransition ] = useTransition();

    const onClose = () => startTransition(() => setOpened(false));

    useEffect(() => {
        if (props.imageId) {
            // TODO: Get the preview image.
        }
        startTransition(() => {
            if (props.show) {
                console.log('show:', props.show);
                setOpened(props.show);
            }

            if (props.imageId) {
                setImageId(props.imageId);
            }
        });
    }, [ props.show, props.imageId ]);

    return <Fragment>
        <StyledBackdrop open={ opened } onClick={ onClose }>
            <ImageSection>
                {
                    props.imageId ?
                        <ImageIcon style={{ fontSize: '5rem', color: '#ffffff' }} />
                        :
                        <ImageNotSupported style={{ fontSize: '5rem', color: '#ffffff' }} />
                }
            </ImageSection>

            <ControlSection>
                <IconButton aria-label="zoom in">
                    <ZoomIn style={{ fontSize: '2rem', color: '#ffffff' }} />
                </IconButton>

                <IconButton aria-label="zoom out">
                    <ZoomOut style={{ fontSize: '2rem', color: '#ffffff' }} />
                </IconButton>
            </ControlSection>
        </StyledBackdrop>
    </Fragment>;
}

export default ImagePreview;

