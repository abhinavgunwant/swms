import {
    useEffect, useState, useRef, useTransition, ChangeEvent, useCallback,
    KeyboardEvent
} from 'react';
import {
    Box, TextField, List, ListItem
} from '@mui/material';

import { throttle } from 'lodash';

import { styled } from '@mui/material/styles';

const SelectedListItem = styled(ListItem)`
    background: #dddddd;
`;

interface TypeaheadProps {
    placeholder?: string,
    dataSource?: "fetch" | "static", // default: "static"
    list?: any[],
    fetcherFunction?: Function,
    onItemSelected: (item: any) => void,
}

const TextFieldFullWidth = styled(TextField)`
    width: 100%;
`;

const OverlayList = styled(Box)`
    background: #ffffff;
    position: fixed;
    z-index: 1;
    box-shadow: 0 0 5px #aaaaaa;
    border-radius: 5px;
`;

const Typeahead = (props: TypeaheadProps) => {
    const [ _pending, startTransition ] = useTransition();

    const [ text, setText ] = useState<string>('');
    const [ list, setList ] = useState<any[]>([]);
    const [ selectionIndex, setSelectionIndex ] = useState<number>(0);
    const [ showOverlayList, setShowOverlayList ] = useState<boolean>(false);
    const [ width, setWidth ] = useState<number>(100);
    const textFieldRef = useRef<HTMLDivElement>(null);
    const parentRef = useRef<HTMLDivElement>(null);
    const overlayListRef = useRef<HTMLUListElement>(null);

    const hideOverlayList = useCallback(
        () => startTransition(() => setShowOverlayList(false)
    ), []);

    const queryAPI = async (queryText: string) => {
        console.log('Querying fetcher function with query text: ' + queryText);

        if (props.fetcherFunction) {
            const newList = await props.fetcherFunction(queryText);
            console.log('got new list: ' + newList);
            startTransition(() => setList(newList));
        }
    }

    const throttledQueryAPI = useCallback(throttle(
        (q: string) => queryAPI(q),
        800,
        { trailing: true, leading: false }
    ), []);

    const onFocus = () => {
        startTransition(() => {
            setShowOverlayList(true);
        })
    };

    const onHoverOverItem = (i: number) => setSelectionIndex(i);

    const onOutsideClicked = (event: MouseEvent) => {
        if (!parentRef.current?.contains(event.target as HTMLDivElement)) {
            hideOverlayList();
        }
    };

    const onTextChanged = (event: ChangeEvent<HTMLInputElement>) => {
        setText(event.target.value);

        if (event.target.value) {
            throttledQueryAPI(event.target.value);
        }
    }

    const onKeyDown = (event: KeyboardEvent<HTMLInputElement>) => {
        switch(event.key) {
            case 'ArrowUp':
                event.preventDefault();

                if (list?.length) {
                    if (selectionIndex > 0) {
                        startTransition(
                            () => setSelectionIndex(selectionIndex - 1)
                        );
                    } else {
                        startTransition(
                            () => setSelectionIndex(list.length - 1)
                        );
                    }
                } else {
                    startTransition(() => setSelectionIndex(-1));
                }
                break;
            case 'ArrowDown':
                event.preventDefault();

                if (list?.length) {
                    if (selectionIndex < list.length - 1) {
                        startTransition(
                            () => setSelectionIndex(selectionIndex + 1)
                        );
                    } else {
                        startTransition(() => setSelectionIndex(0));
                    }
                } else {
                    startTransition(() => setSelectionIndex(-1));
                }
                break;
            case 'Enter':
                event.preventDefault();
                props.onItemSelected(list[selectionIndex]);
                setShowOverlayList(false);
                break;

            default:
                if (!showOverlayList) {
                    setShowOverlayList(true);
                }
        }
    }

    const onResize = () => {
        if (textFieldRef) {
            startTransition(() =>
                setWidth(textFieldRef.current?.offsetWidth || 100)
            );
        }
    };

    const throttledResize = useCallback(throttle(
        () => onResize(),
        500,
        { trailing: true, leading: false }
    ), []);

    useEffect(() => {
        throttledResize();

        document.addEventListener('click', onOutsideClicked);
        window.addEventListener('resize', throttledResize);

        return () => {
            document.removeEventListener('click', onOutsideClicked);
            window.removeEventListener('resize', throttledResize);
        }
    }, []);

    return <Box ref={ parentRef }>
        <TextFieldFullWidth
            placeholder={ props.placeholder || '' }
            ref={ textFieldRef }
            onFocus={ onFocus }
            onBlur={ hideOverlayList }
            onChange={ onTextChanged }
            onKeyDown={ onKeyDown }
            autoComplete="off" />

        {
            showOverlayList &&
            <OverlayList ref={ overlayListRef } sx={{ width }}>
                <List>
                    {
                        Array.isArray(list) && list.map((item, i) =>
                            i === selectionIndex ?
                                <SelectedListItem
                                    onClick={
                                        () => props.onItemSelected(item)
                                    }
                                    onMouseEnter={ () => onHoverOverItem(i) }
                                    key={ item.id }>
                                    { item.name }
                                </SelectedListItem>
                            :
                                <ListItem
                                    onClick={
                                        () => props.onItemSelected(item)
                                    }
                                    onMouseEnter={ () => onHoverOverItem(i) }
                                    key={ item.id }>
                                    { item.name }
                                </ListItem>
                        )
                    }
                </List>
            </OverlayList>
        }
    </Box>
}

export default Typeahead;

