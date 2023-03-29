import { ReactNode } from 'react';

import { Fab } from '@mui/material';
import styled from '@emotion/styled';
import { styled as muiStyled } from '@mui/material/styles';

const WorkspaceFabWrapper = styled.div`
    position: fixed;
    bottom: 1rem;
    right: 1rem;
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
`;

const StyledFab = muiStyled(Fab)`margin-left: 1rem`;

const FabText = styled.div`
    margin-left: 0.5rem;
`;

interface FabProps {
    icon: ReactNode,
    text: string,
    onClick: () => void,
    show: boolean,
    variant?: "extended" | "circular",
    color?: "primary" | "secondary" | "error",
}

interface WorkspaceFabProps {
    inWorkspaceHome?: boolean,
    fabs: FabProps[],
}

export const WorkspaceFab = (props: WorkspaceFabProps) => <WorkspaceFabWrapper>
    {
        props?.fabs?.map((fab, i) => fab.show ? <StyledFab
            variant={ fab.variant }
            color={ fab.color }
            onClick={ fab.onClick }
            key={ i }>

            { fab.icon }
            <FabText>{ fab.text }</FabText>
        </StyledFab> : '')
    }
</WorkspaceFabWrapper>;

export default WorkspaceFab;

