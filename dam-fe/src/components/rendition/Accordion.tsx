import { useEffect, useState, ChangeEvent, useTransition } from 'react';

import {
    Typography, IconButton, Accordion as AccordionMat, AccordionSummary,
    AccordionDetails, Checkbox, FormGroup, FormControlLabel, List,
} from '@mui/material';

import { Add, ExpandMore } from '@mui/icons-material';

import { Item as RenditionItem } from '.';

import Rendition from '../../models/Rendition';

interface AccordionProps {
    expand?: boolean,
    showPreview?: boolean,
    renditionList: Rendition[],
    showEagerCheckbox?: boolean,
    eagerRendition: boolean,

    onURLCopy?: (rendition: Rendition) => void,
    onRenditionPreview?: (slug: string) => void,
    onEditRendition: (indx: number) => void,
    onDeleteRendition: (indx: number) => void,
    onRenditionClicked: () => void,
    onEagerRenditionChecked: (e: ChangeEvent<HTMLInputElement>) => void,
}

export const Accordion = (
    {
        expand, showPreview, renditionList, showEagerCheckbox, eagerRendition,
        onEditRendition, onDeleteRendition, onRenditionClicked,
        onEagerRenditionChecked, onRenditionPreview, onURLCopy,
    } : AccordionProps
) => {
    const [ expanded, setExpanded ] = useState<boolean>(false);

    const [ _, startTransition ] = useTransition();

    const onSummaryClicked = () => startTransition(
        () => setExpanded(!expanded)
    );

    const onShowPreview = (slug: string) => {
        if (onRenditionPreview) {
            onRenditionPreview(slug);
        }
    };

    useEffect(() => {
        if (expand) {
            setExpanded(true);
        }
    }, []);

    return <AccordionMat expanded={ expanded }>
        <AccordionSummary
            expandIcon={ <ExpandMore /> }
            aria-controls="panel1a-content"
            sx={{ background: '#efefef' }}
            onClick={ onSummaryClicked }>
            <Typography variant="h6">Renditions</Typography>
        </AccordionSummary>

        <AccordionDetails>
            {
                renditionList.length ?
                    <List>
                    {
                        renditionList.map((r: Rendition, i: number) =>
                            <RenditionItem
                                showPreview={ showPreview }
                                onShowPreview={ () => onShowPreview(r.slug) }
                                rendition={ r }
                                onEditRendition={() => onEditRendition(i)}
                                onDeleteRendition={() => onDeleteRendition(i)}
                                onURLCopy={ onURLCopy }
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

