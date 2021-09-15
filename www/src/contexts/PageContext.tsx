import * as React from 'react';

type PageContextType = {
  user: any;
  setUser: (user: any) => void;
};

const GITHUB_USER_KEY = 'gh-user';

const PageContext = React.createContext<PageContextType>({
  user: null,
  setUser: (_) => {},
});

export const PageProvider = ({ children }) => {
  const [user, setUser] = React.useState(() => {
    if (typeof window !== 'undefined') {
      return JSON.parse(sessionStorage.getItem(GITHUB_USER_KEY));
    }
    return null;
  });
  return (
    <PageContext.Provider
      value={{
        user,
        setUser: (user) => {
          setUser(user);
          if (user) {
            sessionStorage.setItem(GITHUB_USER_KEY, JSON.stringify(user));
          } else {
            sessionStorage.removeItem(GITHUB_USER_KEY);
          }
        },
      }}
    >
      {children}
    </PageContext.Provider>
  );
};

export default PageContext;
