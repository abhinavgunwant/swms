import { useEffect, useState, useRef, useTransition } from 'react';
import {
    Box, TextField, List, ListItem
} from '@mui/material';

import { styled } from '@mui/material/styles';

interface TypeaheadProps {
    placeholder?: string,
    dataSource?: "fetch" | "static", // default: "static"
    list?: any[],
    dataSourceUrl?: string,
}

const TextFieldFullWidth = styled(TextField)`
    width: 100%;
`;

const OverlayList = styled(Box)`
    background: #ffffff;
    position: fixed;
    z-index: 50;
    box-shadow: 0 0 5px #aaaaaa;
    border-radius: 5px;
`;

const Typeahead = (props: TypeaheadProps) => {
    const [ _, startTransition ] = useTransition();

    const [ list, setList ] = useState<any[]>();
    const [ showOverlayList, setShowOverlayList ] = useState<boolean>(false);
    const [ width, setWidth ] = useState<number>(100);
    const textFieldRef = useRef<HTMLDivElement>(null);
    const parentRef = useRef<HTMLDivElement>(null);
    const overlayListRef = useRef<HTMLUListElement>(null);

    const onFocus = () => {
        startTransition(() => {
            setShowOverlayList(true);
        })
    };

    const onOutsideClicked = (event: MouseEvent) => {
        if (!parentRef.current?.contains(event.target as HTMLDivElement)) {
            setShowOverlayList(false);
        }
    };

    useEffect(() => {
        if (textFieldRef) {
            startTransition(() =>
                setWidth(textFieldRef.current?.offsetWidth || 100)
            );
        }

        document.addEventListener('click', onOutsideClicked);

        return () => {
            document.removeEventListener('click', onOutsideClicked);
        }
    }, []);

    return <Box ref={ parentRef }>
        <TextFieldFullWidth
            placeholder={ props.placeholder || '' }
            ref={ textFieldRef }
            onFocus={ onFocus }
            onBlur={
                () => startTransition(() => setShowOverlayList(false))
            } />

            {
                showOverlayList &&
                <OverlayList ref={ overlayListRef } sx={{ width }}>
                    <List>
                        <ListItem>Item 1</ListItem>
                    </List>
                </OverlayList>
            }
    </Box>
}

export default Typeahead;
