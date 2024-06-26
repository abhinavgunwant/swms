import { useEffect, useRef, useState, useTransition } from 'react';

import {
    Grid, Card, CardActions, CardContent, CardMedia, IconButton, Typography,
    ClickAwayListener, Tooltip,
} from '@mui/material';

import { Folder, FolderSpecial } from '@mui/icons-material';

import ThumbnailExtendedProps, { ThumbnailAction } from '../models/ThumbnailExtendProps';

import { styled } from '@mui/material/styles';

const ThumbnailSubtitle = styled(Typography)`color: #888888;`;

const ThumbnailActions = styled(CardActions)`
    display: flex;
    justify-content: center;
    padding-top: 0;
    background: #ffffff;
    transition: 0.25s background ease-out;

    &:hover {
        background: #f8f8f8;
    }
`;

const CustomCardMedia = styled(CardMedia)`
    height: 200px;
    display: flex;
    justify-content: center;
    align-items: center;
`;

const DEFAULT_IMAGE_THUMBNAIL ="/logo192.png"; 

export const Thumbnail = (props: ThumbnailExtendedProps) => {
    const [ showThumbnail, setShowThumbnail ] = useState<boolean>();

    const [ _, startTransition ] = useTransition();

    const cardRef = useRef<HTMLDivElement|null>(null);
    const imgRef = useRef<HTMLImageElement|null>(null);
    const intObserverRef = useRef<IntersectionObserver>();

    const onClickAway = (action: ThumbnailAction) => {
        if (action && action.onHideTooltip) {
            action.onHideTooltip();
        }
    };

    useEffect(() => {
        if (props.thumbnailLocation) {
            intObserverRef.current = new IntersectionObserver(
                (entries: IntersectionObserverEntry[]) => {
                    entries.forEach((entry) => {
                        if (entry.intersectionRatio > 0.2) {
                            startTransition(() => setShowThumbnail(true));

                            if (intObserverRef.current) {
                                intObserverRef.current.disconnect();
                            }
                        }
                    });
            })

            if (cardRef.current) {
                intObserverRef.current.observe(cardRef.current);
            }
        }

        return () => {
            if (intObserverRef.current) {
                try {
                    intObserverRef.current.disconnect();
                } catch (e) { console.log(e); }
            }
        }
    }, []);

    useEffect(() => {
        setTimeout(() => {
            if (showThumbnail && !(
                imgRef.current && imgRef.current.complete
                && imgRef.current.naturalHeight
            )) {
                startTransition(() => setShowThumbnail(false));
            }
        }, 10000);
    }, [ showThumbnail ]);

    return <Grid item xs={12} sm={6} lg={3} xl={2}>
        <Card
            variant="outlined"
            style={{
                backgroundColor: props.selected ? '#1976d244' : 'transparent',
                boxShadow: props.selected ? '0 0 0.5rem #1976d244' : 'none',
            }}
            onClick={ props.onClick }
            ref={ cardRef }>

            {
                !props.type || props.type === "IMAGE" ?
                    <CardMedia
                        component="img"
                        height="200"
                        image={
                            showThumbnail && props.thumbnailLocation
                            || DEFAULT_IMAGE_THUMBNAIL
                        }
                        alt={ props.name }
                        ref={ imgRef } />
                :
                    props.type === "PROJECT" ?
                    <CustomCardMedia>
                        <FolderSpecial
                            color="primary"
                            sx={{ fontSize: '80px' }} />
                    </CustomCardMedia>
                    :
                    <CustomCardMedia>
                        <Folder
                            color="primary"
                            sx={{ fontSize: '80px' }} />
                    </CustomCardMedia>
            }

            <CardContent>
                <Typography variant="h5">
                    { props.name }
                </Typography>
                <ThumbnailSubtitle>
                    { props.subtitle }
                </ThumbnailSubtitle>
            </CardContent>

            <ThumbnailActions disableSpacing>
                {
                    props?.actions?.map((action, i) => {
                        if (action.tooltip && action.show) {
                            return <ClickAwayListener
                                onClickAway={ () => onClickAway(action) }>
                                <Tooltip
                                    title={ action.tooltip }
                                    open={ action.showTooltip }
                                    onClose={ action.onHideTooltip }
                                    placement="top"
                                    disableFocusListener
                                    disableHoverListener>
                                    <IconButton
                                        aria-label={ action.label }
                                        onClick={ (e) => action?.action(e) }
                                        key={ i }>
                                        { action.icon }
                                    </IconButton>
                                </Tooltip>
                            </ClickAwayListener>;
                        }

                        if (action.show) {
                            return <IconButton
                                aria-label={ action.label }
                                onClick={ (e) => action?.action(e) }
                                key={ i }>
                                { action.icon }
                            </IconButton>;
                        }
                    })
                }
            </ThumbnailActions>
        </Card>
    </Grid>;
}

export default Thumbnail;

