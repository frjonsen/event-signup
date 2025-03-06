import PublicViewHeader from './common/PublicViewHeader';
import { useAuth } from 'react-oidc-context';
import useMediaQuery from '@mui/material/useMediaQuery';

function App() {
  const auth = useAuth();
  const prefersDarkMode = useMediaQuery('(prefers-color-scheme: dark)');

  return (<>
    <PublicViewHeader />
    {auth?.isAuthenticated ? <p>{auth.user?.profile.email}</p> : <></>}
    <span>Prefers dark mode: {prefersDarkMode.toString()}</span>
  </>)
}

export default App
