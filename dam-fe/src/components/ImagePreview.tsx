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

export const ImagePreview = (props: ImagePreviewProps) => {
    const [ loading, setLoading ] = useState<boolean>(true);
    const [ imagePath, setImagePath ] = useState<string>('');
    const [ imageId, setImageId ] = useState<number>(props?.imageId || -1);
    const [ zoom, setZoom ] = useState<number>(100);

    const [ _, startTransition ] = useTransition();

    const imageRef = useRef<HTMLImageElement>(null);
    const imgSectionRef = useRef<HTMLDivElement>(null);

    const onImageLoaded = () => {
        onResize();

        setLoading(false);
    }

    const onResize = () => {
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
    }

    useEffect(() => {
        if (!props.show) {
            setLoading(true);
        }
    }, [ props.show ]);

    useEffect(() => {
        document.addEventListener('resize', onResize);

        return () => {
            document.removeEventListener('resize', onResize);
        }
    }, []);

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
                    onClick={ (e) => { console.log('Zoom In!'); } }>
                    <ZoomIn style={{ fontSize: '2rem', color: '#ffffff' }} />
                </ControlButton>

                <ControlButton
                    aria-label="zoom out"
                    onClick={ (e) => { console.log('Zoom Out'); } }>
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

