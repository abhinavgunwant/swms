import { expect, vi } from 'vitest';
import { render, screen } from '../utils/test-utils';
import userEvent from '@testing-library/user-event';

import SemiEditableTextField from './SemiEditableTextField';

describe('Semi editable text field', () => {
    it('Loads with label and edit button.', async () => {
        render(<SemiEditableTextField label="This is test label" />);

        expect(
            screen.getAllByText("This is test label")[0]
        ).toBeInTheDocument();

        expect(screen.getByTestId('setf-edit-button')).toBeInTheDocument();
    });

    it('Loads with label, value and edit button.', async () => {
        render(<SemiEditableTextField label="Test label" value="testval" />);

        expect(screen.getAllByText("Test label")[0]).toBeInTheDocument();

        expect(screen.getByTestId('setf-edit-button')).toBeInTheDocument();
        expect(screen.getByDisplayValue('testval')).toBeInTheDocument();
    });

    it('Shows save and cancel button when edit button is clicked.', async () => {
        render(<SemiEditableTextField label="This is test label" />);

        const editButton = screen.getByTestId('setf-edit-button');

        userEvent.click(editButton);

        const saveButton = screen.getByTestId('setf-save-button');
        const cancelButton = screen.getByTestId('setf-cancel-button');

        expect(screen.queryByText('setf-edit-button')).toBeNull();
        expect(saveButton).toBeInTheDocument();
        expect(cancelButton).toBeInTheDocument();
    });

    it('Is editable when edit button is clicked.', async () => {
        render(<SemiEditableTextField label="This is test label" />);

        userEvent.click(screen.getByTestId('setf-edit-button'));

        userEvent.click(screen.getByTestId('setf-input'));
        userEvent.keyboard('updated');
        expect(screen.getAllByDisplayValue('updated')[0]).toBeInTheDocument();
    });

    it('Discards edited value when cancel button is clicked.', async () => {
        render(<SemiEditableTextField label="testing edit" value="test" />);

        userEvent.click(screen.getByTestId('setf-edit-button'));

        userEvent.click(screen.getByTestId('setf-input'));
        userEvent.keyboard('updated');

        expect(screen.queryByDisplayValue('test')).toBeNull();
        expect(screen.getByDisplayValue('updated')).toBeInTheDocument();

        userEvent.click(screen.getByTestId('setf-cancel-button'));

        expect(screen.queryByDisplayValue('updated')).toBeNull();
        expect(screen.getByDisplayValue('test')).toBeInTheDocument();
    });

    it('Persists edited value when save button is clicked.', async () => {
        render(<SemiEditableTextField label="testing edit" value="test" />);

        userEvent.click(screen.getByTestId('setf-edit-button'));

        userEvent.click(screen.getByTestId('setf-input'));
        userEvent.keyboard('updated');

        userEvent.click(screen.getByTestId('setf-save-button'));

        expect(screen.getByDisplayValue('updated')).toBeInTheDocument();
        expect(screen.queryByDisplayValue('test')).toBeNull();
    });

    it('Calls `onSave` and `onEdited` when edited text is saved.', async () => {
        const testFuncObj = {
            onSave: (s: string) => s,
            onEdited: (edited: boolean) => edited,
        };

        const spyOnSave = vi.spyOn(testFuncObj, 'onSave');
        const spyOnEdited = vi.spyOn(testFuncObj, 'onEdited');

        render(
            <SemiEditableTextField
                label="testing edit"
                value="test"
                onEdited={ testFuncObj.onEdited }
                onSave={ testFuncObj.onSave } />
        );

        userEvent.click(screen.getByTestId('setf-edit-button'));

        userEvent.click(screen.getByTestId('setf-input'));
        userEvent.keyboard('updated');

        userEvent.click(screen.getByTestId('setf-save-button'));

        expect(spyOnSave).toHaveBeenCalledOnce();
        expect(spyOnSave).toHaveReturnedWith('updated');
        expect(spyOnEdited).toHaveBeenCalledOnce();
        expect(spyOnEdited).toHaveReturnedWith(true);
    });

    // i.e. when user clicks edit, types new value, clicks save, clicks edit
    // again and clicks cancel.
    it('Calls `onEdited` with false when saved value is discarded.', async () => {
        const testFuncObj = {
            onEdited: (edited: boolean) => edited,
        };

        const spyOnEdited = vi.spyOn(testFuncObj, 'onEdited');

        render(
            <SemiEditableTextField
                label="testing edit"
                value="test"
                onEdited={ testFuncObj.onEdited } />
        );

        userEvent.click(screen.getByTestId('setf-edit-button'));

        userEvent.click(screen.getByTestId('setf-input'));
        userEvent.keyboard('updated');

        userEvent.click(screen.getByTestId('setf-save-button'));

        expect(spyOnEdited).toHaveBeenCalledOnce();
        expect(spyOnEdited).toHaveReturnedWith(true);

        userEvent.click(screen.getByTestId('setf-edit-button'));
        userEvent.click(screen.getByTestId('setf-cancel-button'));

        expect(spyOnEdited).toHaveBeenCalledTimes(2);
        expect(spyOnEdited).toHaveReturnedWith(false);
    });
});

