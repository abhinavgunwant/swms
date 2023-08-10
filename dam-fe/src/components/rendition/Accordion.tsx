import { ChangeEvent } from 'react';

import {
    Typography, IconButton, Accordion as AccordionMat, AccordionSummary,
    AccordionDetails, Checkbox, FormGroup, FormControlLabel, List,
} from '@mui/material';

import { Add, ExpandMore } from '@mui/icons-material';

import { Item as RenditionItem } from '.';

import Rendition from '../../models/Rendition';

interface AccordionProps {
    renditionList: Rendition[],
    showEagerCheckbox?: boolean,
    eagerRendition: boolean,

    onEditRendition: (indx: number) => void,
    onDeleteRendition: (indx: number) => void,
    onRenditionClicked: () => void,
    onEagerRenditionChecked: (e: ChangeEvent<HTMLInputElement>) => void,
}

export const Accordion = (
    {
        renditionList, showEagerCheckbox, eagerRendition, onEditRendition,
        onDeleteRendition, onRenditionClicked, onEagerRenditionChecked,
    } : AccordionProps
) => {
    return <AccordionMat>
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
                        renditionList.map((r: Rendition, i: number) =>
                            <RenditionItem
                                rendition={ r }
                                onEditRendition={() => onEditRendition(i)}
                                onDeleteRendition={() => onDeleteRendition(i)}
                                key={ i } />
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

            {
                showEagerCheckbox &&
                <FormGroup>
                    <FormControlLabel
                        control={
                            <Checkbox
                                checked={ eagerRendition }
                                onChange={ onEagerRenditionChecked } />
                        }
                        label="Eagerly create renditions" />
                </FormGroup>
            }
        </AccordionDetails>
    </AccordionMat>;
};

export default Accordion;

