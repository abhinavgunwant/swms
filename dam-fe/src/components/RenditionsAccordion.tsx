import { ChangeEvent } from 'react';

import {
    Typography, IconButton, Accordion, AccordionSummary, AccordionDetails,
    Checkbox, FormGroup, FormControlLabel, List, ListItem, ListItemText,
    ListItemSecondaryAction
} from '@mui/material';

import { Edit, Add, ExpandMore, Delete } from '@mui/icons-material';

import Rendition from '../models/Rendition';

import emoStyled from '@emotion/styled';

const SubText = emoStyled.span`
    color: #888888;
    margin-left: 1rem;
`;

interface RenditionsAccordionProps {
    renditionList: Rendition[],
    eagerRendition: boolean,

    onEditRendition: (indx: number) => void,
    onDeleteRendition: (indx: number) => void,
    onRenditionClicked: () => void,
    onEagerRenditionChecked: (e: ChangeEvent<HTMLInputElement>) => void,
}

export const RenditionsAccordion = (
    {
        renditionList, eagerRendition, onEditRendition, onDeleteRendition,
        onRenditionClicked, onEagerRenditionChecked,
    } : RenditionsAccordionProps
) => {
    return <Accordion>
        <AccordionSummary
            expandIcon={ <ExpandMore /> }
            aria-controls="panel1a-content"
            sx={{ background: '#efefef' }}>
            <Typography variant="h6">Renditions</Typography>
        </AccordionSummary>

        <AccordionDetails>
            {
                renditionList.length ?
                    <List>
                    {
                        renditionList.map(
                            (rendition: Rendition, i) => <ListItem
                                key={ i }>
                                <ListItemText>
                                    <Typography>
                                        { rendition.targetDevice }
                                        <SubText>
                                            ({ rendition.slug },
                                            {' '}
                                            {rendition.encoding },
                                            {' '}
                                            { rendition.width }x
                                            { rendition.height })
                                        </SubText>
                                    </Typography>
                                </ListItemText>

                                <ListItemSecondaryAction>
                                    <IconButton
                                        onClick={ () => {
                                            onEditRendition(i);
                                        }}>
                                        <Edit />
                                    </IconButton>

                                    <IconButton
                                        color="error"
                                        onClick={ () => {
                                            onDeleteRendition(i);
                                        }}>
                                        <Delete />
                                    </IconButton>
                                </ListItemSecondaryAction>
                            </ListItem>
                        )
                    }
                    </List>
                    :
                    <Typography>No Renditions</Typography>
            }

            <IconButton
                color="secondary"
                onClick={ onRenditionClicked }>
                <Add />
            </IconButton>

            <FormGroup>
                <FormControlLabel
                    control={
                        <Checkbox
                            checked={ eagerRendition }
                            onChange={ onEagerRenditionChecked } />
                    }
                    label="Eagerly create renditions" />
            </FormGroup>
        </AccordionDetails>
    </Accordion>;
};

export default RenditionsAccordion;

