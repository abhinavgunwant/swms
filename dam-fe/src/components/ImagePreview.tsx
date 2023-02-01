import { Fragment, useState, useEffect, useTransition, useRef } from 'react';
import { Backdrop, IconButton, CircularProgress } from '@mui/material';
import {
    Image as ImageIcon, ImageNotSupported, ZoomIn, ZoomOut, Close
} from '@mui/icons-material';

import styled from '@emotion/styled';
import { styled as muiStyled } from '@mui/material/styles';

const StyledBackdrop = muiStyled(Backdrop)`
    display: flex;
    flex-direction: column;
    z-index: 1101;
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
    gap: 0.5rem;

    width: 100%;
    height: 100px;
`;

const ControlButton = muiStyled(IconButton)`
    background: #aaaaaaaa;

    &:hover {
        box-shadow: 0 0 10px #ffffff;
    }
`;

const CloseButton = muiStyled(ControlButton)`
    position: fixed;
    right: 1rem;
    top: 1rem;
`;

interface ImagePreviewProps {
    imageId?: number,
    show: boolean,
    onClose: () => void,
}

/**
 * Tells how the image will be sized.
 * default - Loads image in it's original size and allows zoom-in and zoom-out.
 * original - Resizes image to it's original size.
 * screen - Fits the image to the screen.
 *
 * When the image fit is in either `original` or `fit` and it is resized, the image
 * fit is moved to `default`.
 */
type ImageFit = 'default' | 'original' | 'screen';

const ZOOM_INCREMENT = 0.25;

export const ImagePreview = (props: ImagePreviewProps) => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ originalWidth, setOriginalWidth ] = useState<number>();
    const [ originalHeight, setOriginalHeight ] = useState<number>();
    const [ imageFit, setImageFit ] = useState<ImageFit>('default');
    const [ imagePath, setImagePath ] = useState<string>('');
    const [ imageId, setImageId ] = useState<number>(props?.imageId || -1);
    const [ zoom, setZoom ] = useState<number>(1);

    const [ _, startTransition ] = useTransition();

    const imageRef = useRef<HTMLImageElement>(null);
    const imgSectionRef = useRef<HTMLDivElement>(null);

    const onImageLoaded = () => {
        setLoading(false);

        if (
            typeof imageRef !== 'undefined'
            && typeof imgSectionRef !== 'undefined'
        ) {
            const imgWidth = imageRef.current?.width;
            const imgHeight = imageRef.current?.height;

            if (
                typeof imgWidth !== 'undefined'
                && typeof imgHeight !== 'undefined'
                && imageRef.current
            ) {
                setOriginalWidth(imageRef.current.width);
                setOriginalHeight(imageRef.current.height);
            }
        }
    }

    const imageFitToWidth = () => {
        if (
            typeof imageRef === 'undefined'
            //&& typeof imageRef.current === 'undefined'
            && typeof imgSectionRef === 'undefined'
            //&& typeof imgSectionRef.current === 'undefined'
        ) {
            return;
        }

        const imgWidth = imageRef.current?.width;
        const imgHeight = imageRef.current?.height;

        const cWidth = imgSectionRef.current?.clientWidth;
        const cHeight = imgSectionRef.current?.clientHeight;

        if (
            typeof imgWidth === 'undefined'
            || typeof imgHeight === 'undefined'
            || typeof cWidth === 'undefined'
            || typeof cHeight === 'undefined'
        ) {
            return;
        }

        const imgAspectRatio:number = imgWidth / imgHeight;
        const imgSectionAspectRatio:number = cWidth / cHeight;

        console.log('Image width:', imgWidth, ', Image height:', imgHeight);

        const wideImage = imgSectionAspectRatio >= 1;

        if (imgWidth < cWidth && imgHeight < cHeight) {
            if (wideImage) {
                const newHeight = cHeight * 0.9;
                if (imageRef && imageRef.current) {
                    imageRef.current.width = (cWidth/cHeight) * newHeight;
                    imageRef.current.height = newHeight;
                }
            } else {
                const newWidth = cWidth * 0.9
                if (imageRef && imageRef.current) {
                    imageRef.current.width = newWidth;
                    imageRef.current.height = (newWidth/cWidth) * cHeight;
                }
            }
        }
    };

    const onZoom = (inOrOut: 'in' | 'out') => {
        if (inOrOut === 'in') {
            setZoom(zoom + ZOOM_INCREMENT);

            return;
        }

        setZoom(zoom - ZOOM_INCREMENT);
    }

    useEffect(() => {
        if (!props.show) {
            setLoading(true);
        } else {
            setZoom(1);

        }
    }, [ props.show ]);

    useEffect(() => {
        console.log('resizing', zoom);
        if (
            typeof imageRef === 'undefined'
            && typeof imgSectionRef === 'undefined'
        ) {
            return;
        }

        if (imageRef.current && originalWidth && originalHeight) {
            console.log(originalWidth * zoom);
            console.log(originalHeight * zoom);
            imageRef.current.width = originalWidth * zoom;
            imageRef.current.height = originalHeight * zoom;
        }
    }, [ zoom ]);

    if (!props.show) {
        return null;
    }

    return <Fragment>
        <StyledBackdrop open={ props.show } onClick={ () => {} }>
            <ImageSection ref={ imgSectionRef }>
                {
                    props.imageId ?
                    <Fragment>
                        <img
                            src="http://localhost:8080/api/image/my-first-project/cute-doggo.jpg"
                            onLoad={ onImageLoaded }
                            ref={ imageRef } />
                        { loading && <CircularProgress /> }
                    </Fragment>
                        :
                        <ImageNotSupported style={{ fontSize: '5rem', color: '#ffffff' }} />
                }
            </ImageSection>

            <ControlSection>
                <ControlButton
                    aria-label="zoom in"
                    onClick={ (e) => { console.log('Zoom In!'); onZoom('in'); } }>
                    <ZoomIn style={{ fontSize: '2rem', color: '#ffffff' }} />
                </ControlButton>

                <ControlButton
                    aria-label="zoom out"
                    onClick={ (e) => { console.log('Zoom Out'); onZoom('out'); } }>
                    <ZoomOut style={{ fontSize: '2rem', color: '#ffffff' }} />
                </ControlButton>

                {/*
                    This button is rendered at the top-right of the screen
                */}
                <CloseButton
                    aria-label="close preview"
                    onClick={ (e) => { props.onClose(); } }>
                    <Close style={{ fontSize: '2rem', color: '#ffffff' }} />
                </CloseButton>
            </ControlSection>
        </StyledBackdrop>
    </Fragment>;
}

export default ImagePreview;

