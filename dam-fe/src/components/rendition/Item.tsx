import { MouseEventHandler, useState, useTransition } from 'react';

import {
    Typography, IconButton, ListItem, ListItemText, ListItemSecondaryAction,
    Tooltip, ClickAwayListener,
} from '@mui/material';

import { Edit, Delete, Visibility, ContentCopy  } from '@mui/icons-material';

import Rendition from '../../models/Rendition';

import emoStyled from '@emotion/styled';

const SubText = emoStyled.span`
    color: #888888;
    margin-left: 1rem;
`;

interface ItemProps {
    rendition: Rendition,
    showPreview?: boolean,
    onURLCopy?: (rendition: Rendition) => void,
    onShowPreview?: () => void,
    onEditRendition: () => void,
    onDeleteRendition: () => void,
}

export const Item = (props: ItemProps) => {
    const [ showTooltip, setShowTooltip ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const onEdit: MouseEventHandler = () => props.onEditRendition();
    const onDelete: MouseEventHandler = () => props.onDeleteRendition();

    const onShowPreview = () => {
        if (props.onShowPreview) {
            props.onShowPreview();
        }
    };

    /**
     * Hides the "URL Copied!" tooltip.
     */
    const hideTooltip = () => startTransition(() => setShowTooltip(false));

    /**
     * Copies the URL.
     */
    const copyURL = () => {
        if (props.onURLCopy) {
            props.onURLCopy(props.rendition);

            startTransition(() => setShowTooltip(true));

            setTimeout(hideTooltip, 2000);
        }
    };

    return <ListItem>
        <ListItemText>
            <Typography>
                { props.rendition.slug }
                <SubText>
                    ({
                        props.rendition.targetDevice ?
                            props.rendition.targetDevice + ', ' : ''
                    }
                    { props.rendition.encoding },{' '}
                    { props.rendition.width }x
                    { props.rendition.height })
                </SubText>
            </Typography>
        </ListItemText>

        <ListItemSecondaryAction>
            {
                props.onURLCopy &&
                <ClickAwayListener
                    onClickAway={ hideTooltip }>
                    <Tooltip
                        title="URL Copied!"
                        open={ showTooltip }
                        onClose={ hideTooltip }
                        placement="top"
                        disableFocusListener
                        disableHoverListener>
                        <IconButton
                            onClick={ copyURL }>
                            <ContentCopy />
                        </IconButton>
                    </Tooltip>
                </ClickAwayListener>
            }

            {
                props.showPreview &&
                <IconButton onClick={ onShowPreview }>
                    <Visibility />
                </IconButton>
            }

            <IconButton
                color="secondary"
                onClick={ onEdit }>
                <Edit />
            </IconButton>

            <IconButton color="error" onClick={ onDelete }>
                <Delete />
            </IconButton>
        </ListItemSecondaryAction>
    </ListItem>;
};

export default Item;

