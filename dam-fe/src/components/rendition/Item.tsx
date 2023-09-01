import { MouseEventHandler } from 'react';

import {
    Typography, IconButton, ListItem, ListItemText, ListItemSecondaryAction
} from '@mui/material';

import { Edit, Delete } from '@mui/icons-material';

import Rendition from '../../models/Rendition';

import emoStyled from '@emotion/styled';

const SubText = emoStyled.span`
    color: #888888;
    margin-left: 1rem;
`;

interface ItemProps {
    rendition: Rendition,
    onEditRendition: () => void,
    onDeleteRendition: () => void,
}

export const Item = (props: ItemProps) => {
    const onEdit: MouseEventHandler = () => props.onEditRendition();
    const onDelete: MouseEventHandler = () => props.onDeleteRendition();

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
            <IconButton onClick={ onEdit }><Edit /></IconButton>

            <IconButton color="error" onClick={ onDelete }>
                <Delete />
            </IconButton>
        </ListItemSecondaryAction>
    </ListItem>;
};

export default Item;

