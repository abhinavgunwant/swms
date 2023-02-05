import actualCreate, { StateCreator } from 'zustand';
import WorkspaceState from './WorkspaceState';

export const storeResetFns = new Set<() => void>();

export const create = () => <S,>(createState: StateCreator<WorkspaceState>) => {
    const store = actualCreate<WorkspaceState>(createState);
    const initialState = store.getState();
    storeResetFns.add(() => store.setState(initialState, true));
    
    return store;
}

export default create;

it ('workspace store test', () => {
    // TODO: implement this!
});

