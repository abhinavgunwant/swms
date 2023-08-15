import {
    useState, useEffect, useTransition, MouseEvent, ChangeEvent,
} from 'react';

import {
    Dialog, DialogTitle, DialogContent, Button, Grid, TextField as _TextField,
    FormControl, Select, MenuItem, InputLabel, Alert,
} from '@mui/material';

import { SelectChangeEvent } from '@mui/material/Select';

import Rendition from '../../models/Rendition';

import { styled } from '@mui/material/styles';

export type RenditionDialogMode = 'new' | 'edit';

interface NewRenditionDialogProps {
    open: boolean,
    imageId?: number,
    error?:boolean,
    errorMessage?: string,
    errorField?: string,
    mode?: RenditionDialogMode, // default mode to be considered `new`
    renditionToEdit?: Rendition,
    onDialogClosed: (e: MouseEvent<HTMLButtonElement>) => void,
    onRenditionSaved: (rendition: Rendition) => void,

    // Only needed when mode is `edit`
    onRenditionUpdated?: (rendition: Rendition) => void,
}

const StyledDialogContent = styled(DialogContent)`
    width:  400px;
`;

const TextField = styled(_TextField)`
    width: 100%;
    margin-top: 0.5rem;
`;

export const RenditionDialog = (props: NewRenditionDialogProps) => {
    const [ height, setHeight ] = useState<number>(0);
    const [ width, setWidth ] = useState<number>(0);
    const [ targetDevice, setTargetDevice ] = useState<string>('');
    const [ slug, setSlug ] = useState<string>('');
    const [ encoding, setEncoding ] = useState<string>('');

    // eslint-disable-next-line @typescript-eslint/no-unused-vars 
    const [ _, startTransition ] = useTransition();

    const createRendition: () => Rendition = () => {
        const now = (new Date()).toISOString();

        return {
            id: 0,
            imageId: props.imageId || 0,
            height,
            width,
            targetDevice,
            slug,
            isPublished: false,
            encoding,
            createdOn: now,
            createdBy: 0,
            modifiedOn: now,
            modifiedBy: 0,
        }
    };

    const onSaveClicked = () => {
        if (typeof width !== 'number' || typeof height !== 'number') {
            return;
        }

        props.onRenditionSaved(createRendition());
    }

    const onEditClicked = () => {
        if (props.onRenditionUpdated) {
            props.onRenditionUpdated(createRendition());
        }
    }

    const onTargetDeviceChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setTargetDevice(e.target.value);
    const onSlugChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setSlug(e.target.value);
    const onWidthChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setWidth(parseInt(e.target.value));
    const onHeightChanged = (e: ChangeEvent<HTMLInputElement>) =>
        setHeight(parseInt(e.target.value));
    const onEncodingChanged = (e: SelectChangeEvent<string>) =>
        setEncoding(e.target.value);

    const onClose = (e: any) => {
        props.onDialogClosed(e);

        startTransition(() => {
            setHeight(0);
            setWidth(0);
            setTargetDevice('');
            setSlug('');
            setEncoding('');
        });
    };

    useEffect(() => {
        // Refresh the state every time the dialog is opened
        startTransition(() => {
            if (props.mode === 'edit' && props.renditionToEdit) {
                const rte = props.renditionToEdit;

                setHeight(rte.height);
                setWidth(rte.width);
                setTargetDevice(rte.targetDevice);
                setSlug(rte.slug);
                setEncoding(rte.encoding);

                return;
            }

            setHeight(0);
            setWidth(0);
            setTargetDevice('');
            setSlug('');
            setEncoding('');
        });
    }, [ props.open, props.renditionToEdit, props.mode ]);

    return <Dialog
        onClose={ (e) => onClose(e) }
        open={ props.open }>

        <DialogTitle>New Rendition</DialogTitle>

        <StyledDialogContent>
            <Grid container>
                <Grid item xs={ 12 }>
                    <TextField
                        label="Target Device"
                        variant="standard"
                        value={ targetDevice }
                        onChange={ onTargetDeviceChanged } />
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Slug"
                        variant="standard"
                        value={ slug }
                        onChange={ onSlugChanged }
                        error={ props.error && props.errorField == 'slug' }
                        helperText={
                            props.error && (
                                props.errorField == 'slug'
                                && props.errorMessage ?
                                    props.errorMessage
                                :
                                    'Slug should not be empty'
                            ) || 'Choose a unique slug for this image'
                        }
                        required />
                </Grid>

                <Grid item xs={ 12 } sx={{ marginTop: '1rem' }}>
                    <FormControl fullWidth>
                        <InputLabel id="new-image-rendition-encoding">
                            Encoding
                        </InputLabel>

                        <Select
                            labelId="new-image-rendition-encoding"
                            label="Encoding"
                            variant="standard"
                            value={ encoding }
                            onChange={ onEncodingChanged }
                            sx={{ marginTop: '4rem' }}>
                            <MenuItem value="JPG">JPEG</MenuItem>
                            <MenuItem value="PNG">PNG</MenuItem>
                            <MenuItem value="WEBP">WebP</MenuItem>
                        </Select>
                    </FormControl>
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Width"
                        variant="standard"
                        type="number"
                        value={ width }
                        onFocus={ (e) => e.target.select() }
                        onChange={ onWidthChanged } />
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Height"
                        variant="standard"
                        type="number"
                        value={ height }
                        onFocus={ (e) => e.target.select() }
                        onChange={ onHeightChanged } />
                </Grid>

                {
                    props.error &&
                    <Grid item xs={12} sx={{ marginTop: '1rem' }}>
                        <Alert severity="error"> Some error occured! </Alert>
                    </Grid>
                }

                <Grid item sx={{ marginTop: '1rem' }}>
                    {
                        props.mode === 'edit' ?
                            <Button
                                variant="contained"
                                onClick={ onEditClicked }>
                                Edit
                            </Button>
                        :
                            <Button
                                variant="contained"
                                onClick={ onSaveClicked }>
                                Save
                            </Button>
                    }

                    <Button
                        onClick={ (e) => onClose(e) }
                        sx={{ marginLeft: '1rem' }}>
                        Cancel
                    </Button>
                </Grid>
            </Grid>
        </StyledDialogContent>
    </Dialog>;
}

export default RenditionDialog;

