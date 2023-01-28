import { useEffect } from 'react';

import { MenuItem, Skeleton, FormControl, InputLabel } from '@mui/material';

import Select, { SelectChangeEvent } from '@mui/material/Select';

import useAdminStore from '../store/admin/AdminStore';
import useAPI from '../hooks/useAPI';

interface RoleSelectProps {
    userRole: number,
    onChange: (userRole: number) => void
}

export const RoleSelect = (props: RoleSelectProps) => {
    const { getRoles } = useAPI();
    const adminStore = useAdminStore();

    const onChange = (e: SelectChangeEvent) => {
        try {
            props.onChange(parseInt(e.target.value));
        } catch (e) {
            console.log(e);
        }
    };

    useEffect(() => {
        const _getRoles = async () => {
            const roleResp = await getRoles();

            if (roleResp && roleResp.success && roleResp.roles) {
                adminStore.setRoles(roleResp.roles);
                return;
            }

            // TODO: Handle any error here...
        };

        _getRoles();
    }, []);

    if (adminStore.roles.length) {
        console.log('userRole:', props.userRole);

        return <FormControl
            sx={{ margin: '0.5rem 0' }}
            fullWidth>
            <InputLabel id="role-select-user-role-label">User Role</InputLabel>
            <Select
                labelId="role-select-user-role-label"
                id="role-select-user-role"
                label="User Role"
                value={ props.userRole.toString() }
                onChange={ onChange }>
                {
                    adminStore.roles.map((role, i) => <MenuItem
                        key={ i }
                        value={ role.id }>
                        { role.roleName }
                    </MenuItem>)
                }
            </Select>
        </FormControl>
    }

    return <Skeleton
        animation="wave"
        height={ 64 }
        width={ 200 }
    />;
};

export default RoleSelect;

