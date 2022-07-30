import { useState } from 'react';
import { Link } from 'react-router-dom';

import ButtonGroup from '@mui/material/ButtonGroup';
import Button from '@mui/material/Button';
import ViewListIcon from '@mui/icons-material/ViewList';
import GridViewIcon from '@mui/icons-material/GridView';

import Breadcrumbs from '../../components/Breadcrumbs';

import LinkModel from '../../models/LinkModel';
import useWorkspaceStore from '../../store/workspace/WorkspaceStore';
import BreadcrumbProps from '../../models/props/BreadcrumbProps';

import styled from '@emotion/styled';

const PageTopRow = styled.div`
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
`;

const DISPLAY_STYLES = {
    GRID: 'GRID',
    LIST: 'LIST',
};

/**
 * The top row containing breadcrumbs and view options
 * 
 * @param props 
 * @returns 
 */
const WorkspaceTopRow = (props: BreadcrumbProps) => {
    const store = useWorkspaceStore();

    const setGrid = () => store.setDisplayStyle(DISPLAY_STYLES.GRID);
    const setList = () => store.setDisplayStyle(DISPLAY_STYLES.LIST);

    return <PageTopRow>
        <Breadcrumbs {...props} />

        <ButtonGroup>
            <Button
                variant={
                    store.displayStyle === DISPLAY_STYLES.LIST
                        ? 'contained' : 'outlined'
                }
                onClick={ setList }>
                <ViewListIcon />
            </Button>
            <Button
                variant={
                    store.displayStyle === DISPLAY_STYLES.GRID
                        ? 'contained' : 'outlined'
                }
                onClick={ setGrid }>
                <GridViewIcon />
            </Button>
        </ButtonGroup>
    </PageTopRow>;
}

export default WorkspaceTopRow;
