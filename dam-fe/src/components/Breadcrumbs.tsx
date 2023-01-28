import { Link } from 'react-router-dom';

//import MuiLink from '@mui/material/Link';
import MuiBreadcrumbs from '@mui/material/Breadcrumbs';
import BreadcrumbProps from '../models/props/BreadcrumbProps';

import { styled } from '@mui/material/styles';

// To make the workspace and new-image/new-folder breadcrumb height same...
// i.e. to prevent any significant layout shift...
const StyledBreadcrumbs = styled(MuiBreadcrumbs)`height: 2.25rem;`;

export const Breadcrumbs = ({ links }: BreadcrumbProps) => <StyledBreadcrumbs>
    {
        links.map((link, i) => 
            (typeof link === 'string') ?
                <span key={i}>{ link }</span>
                :
                <Link to={link.to} key={ i } style={{ color: '#1976d2' }}>
                    { link.text }
                </Link>
        )
    }
</StyledBreadcrumbs>;

export default Breadcrumbs;

