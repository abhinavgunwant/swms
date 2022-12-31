import { Box, CircularProgress } from '@mui/material';

import { styled } from '@mui/material/styles';

const Wrapper = styled(Box)`
    display: flex;
    justify-content: center;
    align-items: center;

    width: 100%;
    height: 100%;
    min-height: 160px;
`;

export const Loading = () => <Wrapper><CircularProgress /></Wrapper>;

export default Loading;

