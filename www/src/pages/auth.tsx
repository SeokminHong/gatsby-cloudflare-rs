import * as React from 'react';
import { navigate } from 'gatsby';
import Spinner from '../components/Spinner';
import PageContext from '../contexts/PageContext';

const AuthPage = () => {
  const { setUser } = React.useContext(PageContext);
  React.useEffect(() => {
    const fetchUser = async () => {
      const search = new URLSearchParams(location.search);
      const token = search.get('token');

      await fetch(`https://api.github.com/user`, {
        headers: {
          Accept: `application/vnd.github.v3+json`,
          Authorization: `token ${token}`,
        },
      })
        .then((res) => res.json())
        .then((res) =>
          setUser({ name: res.login, avatar_url: res.avatar_url })
        );
    };

    fetchUser()
      .then(() => navigate('/', { replace: true }))
      .catch(() => navigate('/?login=error', { replace: true }));
  }, []);

  return <Spinner />;
};

export default AuthPage;
