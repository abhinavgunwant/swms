import Alert from '@mui/material/Alert';

interface ErrorProps {
    on: boolean,
    children?: React.ReactNode,
}

export const Error = (props: ErrorProps) => {
    if (props.on) {
        return <Alert severity="error"> { props.children } </Alert>;
    }

    return null;
}

export default Error;

