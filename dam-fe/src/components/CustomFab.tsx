import { ReactElement } from 'react';

import { Fab, FabProps } from '@mui/material';

import { styled as muiStyled } from '@mui/material/styles';
import styled from '@emotion/styled';

interface IndividualFabProps {
    text: string,
    color?: FabProps['color'],
    preIcon?: ReactElement,
    postIcon?: ReactElement,
    show?: boolean,
    onClick: () => void,
}

interface CustomFabProps {
    fabs: IndividualFabProps[],
}

const FabWrapper = styled.div`
    display: flex;
    justify-content: flex-end;
    align-items: flex-end;
    flex-direction: row-reverse;

    position: fixed;
    bottom: 1rem;
    right: 1rem;
`;

const StyledFab = muiStyled(Fab)`
    margin-left: 1rem;
`;

const iconStyle = {
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
};

const Icon = (props: any) => {
    if (props.pre) {
        return <span style={{ ...iconStyle, marginRight: '0.5rem' }}>
            { props.pre }
        </span>;
    }

    if (props.post) {
        return <span style={{ ...iconStyle, marginLeft: '0.5rem' }}>
            { props.post }
        </span>;
    }

    return null;
};

export const CustomFab = (props: CustomFabProps) => {
    return <FabWrapper>
        {
            props.fabs
            .filter((fab) => fab.show ? fab.show : false)
            .map((fab, i) => <StyledFab
                key={ i }
                color={ fab.color }
                variant="extended"
                onClick={ fab.onClick }>
                <Icon pre={ fab.preIcon } />
                { fab.text }
                <Icon post={ fab.postIcon } />
            </StyledFab>)
        }
    </FabWrapper>
};

export default CustomFab;

