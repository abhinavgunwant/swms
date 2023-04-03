import { Fragment } from 'react';

import {
    Grid, Card, CardActions, CardContent, CardMedia, IconButton, Typography
} from '@mui/material';

import { Folder, FolderSpecial } from '@mui/icons-material';

import ThumbnailExtendedProps from '../models/ThumbnailExtendProps';

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

export const Thumbnail = (props: ThumbnailExtendedProps) => {
    return <Grid item xs={12} sm={6} lg={3} xl={2}>
        <Card
            variant="outlined"
            style={{
                backgroundColor: props.selected ? '#1976d244' : 'transparent',
                boxShadow: props.selected ? '0 0 0.5rem #1976d244' : 'none',
            }}
            onClick={ props.onClick }>

            {
                !props.type || props.type === "IMAGE" ?
                    <CardMedia
                        component="img"
                        height="200"
                        image="/logo192.png"
                        alt={ props.name } />
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

