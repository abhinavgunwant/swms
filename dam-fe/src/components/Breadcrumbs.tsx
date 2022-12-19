import { Link } from 'react-router-dom';

import MuiLink from '@mui/material/Link';
import MuiBreadcrumbs from '@mui/material/Breadcrumbs';
import BreadcrumbProps from '../models/props/BreadcrumbProps';

import { styled } from '@mui/material/styles';

// To make the workspace and new-image/new-folder breadcrumb height same...
// i.e. to prevent any significant layout shift...
const StyledBreadcrumbs = styled(MuiBreadcrumbs)`
    height: 2.25rem;
`;

const Breadcrumbs = ({ links }: BreadcrumbProps) => {
    return <StyledBreadcrumbs>
        {
            links.map((link, i) => 
                (typeof link === 'string') ?
                    <span key={i}>{ link }</span>
                    :
                    <MuiLink key={i} underline="none"><Link to={link.to}>{ link.text }</Link></MuiLink>
            )
        }
    </StyledBreadcrumbs>;
}

export default Breadcrumbs;

