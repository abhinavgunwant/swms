import {
    useState, useEffect, useTransition, MouseEvent, ChangeEvent, Fragment,
} from 'react';

import {
    Dialog, DialogTitle, DialogContent, Button, Grid, TextField as _TextField,
    FormControl, Select, MenuItem, InputLabel, Alert, CircularProgress,
    Typography,
} from '@mui/material';

import { SelectChangeEvent } from '@mui/material/Select';

import Rendition from '../../models/Rendition';

import { styled } from '@mui/material/styles';

export type RenditionDialogMode = 'new' | 'edit';

type FormValidationField = {
    valid: boolean,
    updated: boolean,
    validationMessage?: string,
};

interface FormValidationError {
    targetDevice: FormValidationField,
    slug: FormValidationField,
    encoding: FormValidationField,
    width: FormValidationField,
    height: FormValidationField,
}

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

const defaultFvError = (): FormValidationError => ({
    targetDevice: { valid: true, updated: false },
    slug: { valid: true, updated: false },
    encoding: { valid: true, updated: false },
    width: { valid: true, updated: false },
    height: { valid: true, updated: false },
});

const StyledDialogContent = styled(DialogContent)`
    width:  400px;
`;

const TextField = styled(_TextField)`
    width: 100%;
    margin-top: 0.5rem;
`;

export const RenditionDialog = (props: NewRenditionDialogProps) => {
    const [ dialogCounter, setDialogCounter ] = useState<number>(0);
    // Used in edited mode to enable save button.
    const [ timeoutError, setTimeoutError ] = useState<boolean>(false);
    const [ fvError, setFvError ] = useState<FormValidationError>(
        defaultFvError()
    );
    const [ showFvErrorBottom, setShowFvErrorBottom ]
        = useState<boolean>(false);
    const [ updated, setUpdated ] = useState<boolean>(false);
    const [ saving, setSaving ] = useState<boolean>(false);
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

    const validateForm = () => {
        let formValidationError: FormValidationError = defaultFvError();
        let error: boolean = false;

        formValidationError.targetDevice.updated = true;
        formValidationError.slug.updated = fvError.slug.updated;
        formValidationError.encoding.updated = fvError.encoding.updated;
        formValidationError.width.updated = fvError.width.updated;
        formValidationError.height.updated = fvError.height.updated;

        if (fvError.slug.updated && !slug) {
            formValidationError.slug = {
                valid: false,
                updated: true,
                validationMessage: 'Slug cannot be empty',
            };

            error = true;
        }

        if (fvError.encoding.updated && !encoding) {
            formValidationError.encoding = {
                valid: false,
                updated: true,
                validationMessage: 'You must select an encoding',
            };

            error = true;
        }

        if (fvError.width.updated && width <= 0) {
            formValidationError.width = {
                valid: false,
                updated: true,
                validationMessage: 'Width should be positive non-zero number',
            };

            error = true;
        }

        if (fvError.height.updated && height <= 0) {
            formValidationError.height = {
                valid: false,
                updated: true,
                validationMessage: 'Height should be positive non-zero number',
            };

            error = true;
        }

        startTransition(() => {
            setFvError(formValidationError);

            if (!error) {
                setShowFvErrorBottom(false);
            }
        });
    };

    const onSaveClicked = () => {
        if (
            typeof width !== 'number' || typeof height !== 'number' || saving
        ) {
            return;
        }

        if (
            !(
                fvError.targetDevice.updated
                && fvError.slug.updated && fvError.encoding.updated
                && fvError.width.updated && fvError.height.updated
                && fvError.targetDevice.valid
                && fvError.slug.valid && fvError.encoding.valid
                && fvError.width.valid && fvError.height.valid
            )
        ) {
            let fve = { ...fvError };
            fve.targetDevice.updated = true;
            fve.slug.updated = true;
            fve.encoding.updated = true;
            fve.width.updated = true;
            fve.height.updated = true;

            startTransition(() => {
                setFvError(fve);
                setShowFvErrorBottom(true)
            });
            return;
        }

        const counter = dialogCounter;

        // if this dialog is not closed within 30 seconds, give the timeout
        // error

        window.setTimeout(() => {
            if (counter === dialogCounter) {
                startTransition(() => setTimeoutError(true))
            }
        }, 30000);

        if (props.mode === 'edit') {
            if (props.onRenditionUpdated) {
                props.onRenditionUpdated(createRendition());
            }
        } else {
            props.onRenditionSaved(createRendition());
        }

        startTransition(() => setSaving(true));
    };

    const onTargetDeviceChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setTargetDevice(e.target.value);

        if (!updated) {
            setUpdated(true);
        }

        if (!fvError.targetDevice.updated) {
            let fve = { ...fvError };
            fve.targetDevice.updated = true;
            setFvError(fve);
        }
    };

    const onSlugChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setSlug(e.target.value);

        if (!updated) {
            setUpdated(true);
        }

        if (!fvError.slug.updated) {
            let fve = { ...fvError };
            fve.slug.updated = true
            setFvError(fve);
        }
    };

    const onWidthChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setWidth(parseInt(e.target.value));

        if (!updated) {
            setUpdated(true);
        }

        if (!fvError.width.updated) {
            let fve = { ...fvError };
            fve.width.updated = true;
            setFvError(fve);
        }
    };

    const onHeightChanged = (e: ChangeEvent<HTMLInputElement>) => {
        setHeight(parseInt(e.target.value));

        if (!updated) {
            setUpdated(true);
        }

        if (!fvError.height.updated) {
            let fve = { ...fvError };
            fve.height.updated = true;
            setFvError(fve);
        }
    };

    const onEncodingChanged = (e: SelectChangeEvent<string>) => {
        setEncoding(e.target.value);

        if (!updated) {
            setUpdated(true);
        }

        if (!fvError.encoding.updated) {
            let fve = { ...fvError };
            fve.encoding.updated = true;
            setFvError(fve);
        }
    };

    const onClose = (e: any) => {
        if (saving && !timeoutError) {
            return;
        }

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
                let fve = defaultFvError();
                fve.targetDevice.updated = true;
                fve.slug.updated = true;
                fve.encoding.updated = true;
                fve.width.updated = true;
                fve.height.updated = true;

                setHeight(rte.height);
                setWidth(rte.width);
                setTargetDevice(rte.targetDevice);
                setSlug(rte.slug);
                setEncoding(rte.encoding);
                setSaving(false);
                setTimeoutError(false);
                setUpdated(false);
                setFvError(fve);
                setShowFvErrorBottom(false);

                return;
            }

            setDialogCounter(dialogCounter + 1);
            setHeight(0);
            setWidth(0);
            setTargetDevice('');
            setSlug('');
            setEncoding('');
            setSaving(false);
            setTimeoutError(false);
            setUpdated(false);
            setFvError(defaultFvError());
            setShowFvErrorBottom(false);
        });
    }, [ props.open, props.renditionToEdit, props.mode ]);

    useEffect(() => {
        validateForm();
    }, [ targetDevice, slug, encoding, width, height, showFvErrorBottom ]);

    return <Dialog
        onClose={ (e) => onClose(e) }
        open={ props.open }>

        <DialogTitle>
            { props.mode === 'edit' ? 'Edit ' : 'New ' } Rendition
        </DialogTitle>

        <StyledDialogContent>
            <Grid container>
                <Grid item xs={ 12 }>
                    <TextField
                        label="Target Device"
                        variant="standard"
                        disabled={ saving }
                        value={ targetDevice }
                        error={
                            (fvError.targetDevice.updated || saving)
                            && !fvError.targetDevice.valid
                        }
                        onChange={ onTargetDeviceChanged } />
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Slug"
                        variant="standard"
                        disabled={ saving }
                        value={ slug }
                        onChange={ onSlugChanged }
                        error={
                            props.error && props.errorField == 'slug'
                            || ((fvError.slug.updated || saving) && !fvError.slug.valid)
                        }
                        helperText={
                            (props.error && (
                                props.errorField == 'slug'
                                && props.errorMessage ?
                                    props.errorMessage
                                :
                                    'Slug should not be empty'
                            ))
                            || ((fvError.slug.updated || saving) && !fvError.slug.valid) ?
                                    fvError.slug.validationMessage
                                :
                                    'Choose a unique slug for this image'
                        }
                        required />
                </Grid>

                <Grid item xs={ 12 } sx={{ marginTop: '1rem' }}>
                    <FormControl fullWidth>
                        <InputLabel
                            id="new-image-rendition-encoding"
                            color={
                                (
                                    fvError.encoding.updated
                                    && !fvError.encoding.valid
                                ) ? 'error' : 'primary'
                            }>
                            Encoding*
                        </InputLabel>

                        <Select
                            labelId="new-image-rendition-encoding"
                            label="Encoding*"
                            variant="standard"
                            disabled={ saving }
                            value={ encoding }
                            onChange={ onEncodingChanged }
                            error={
                                (fvError.encoding.updated || saving)
                                && !fvError.encoding.valid
                            }
                            sx={{ marginTop: '4rem' }}>
                            <MenuItem value="JPG">JPEG</MenuItem>
                            <MenuItem value="PNG">PNG</MenuItem>
                            <MenuItem value="WEBP">WebP</MenuItem>
                        </Select>

                        {
                            (fvError.encoding.updated || saving)
                            && !fvError.encoding.valid &&
                            <Typography
                                variant="caption"
                                color="error">
                                { fvError.encoding.validationMessage }
                            </Typography>
                        }
                    </FormControl>
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Width*"
                        variant="standard"
                        type="number"
                        disabled={ saving }
                        value={ width }
                        error={
                            (fvError.width.updated || saving) && !fvError.width.valid
                        }
                        helperText={
                            !fvError.width.valid &&
                            fvError.width.validationMessage
                        }
                        onFocus={ (e) => e.target.select() }
                        onChange={ onWidthChanged } />
                </Grid>

                <Grid item xs={ 12 }>
                    <TextField
                        label="Height*"
                        variant="standard"
                        type="number"
                        disabled={ saving }
                        value={ height }
                        error={
                            (fvError.height.updated || saving) && !fvError.height.valid
                        }
                        helperText={
                            !fvError.height.valid &&
                            fvError.height.validationMessage
                        }
                        onFocus={ (e) => e.target.select() }
                        onChange={ onHeightChanged } />
                </Grid>

                {
                    (props.error || showFvErrorBottom) &&
                    <Grid item xs={12} sx={{ marginTop: '1rem' }}>
                        <Alert severity="error">
                            Error saving renditions!
                        </Alert>
                    </Grid>
                }

                {
                    timeoutError &&
                    <Grid item xs={ 12 } sx={{ marginTop: '1rem' }}>
                        <Alert severity="warning">
                            It&apos;s taking longer than usual to connect to
                            the server.
                            <br />
                            Please check back later!
                            <br />
                            You can close this dialog by clicking
                            &quot;Close&quot; button below.
                            <br />
                        </Alert>
                    </Grid>
                }

                <Grid item sx={{ marginTop: '1rem' }}>
                    <Button
                        variant="contained"
                        disabled={ !updated }
                        onClick={ onSaveClicked }>
                        {
                            saving ?
                                <Fragment>
                                    <CircularProgress
                                        size={ 16 }
                                        color="secondary"
                                        sx={{
                                            marginRight: '1rem',
                                            color: '#ffffff',
                                        }} />
                                    Saving
                                </Fragment>
                            :
                                'Save'
                        }
                    </Button>

                    <Button
                        disabled={ saving && !timeoutError }
                        onClick={ (e) => onClose(e) }
                        sx={{ marginLeft: '1rem' }}>
                        { timeoutError ? 'Close' : 'Cancel' }
                    </Button>
                </Grid>
            </Grid>
        </StyledDialogContent>
    </Dialog>;
}

export default RenditionDialog;

