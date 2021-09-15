export const fetchUser = async (token: string) => fetch(`https://api.github.com/user`, {
    headers: {
      Accept: `application/vnd.github.v3+json`,
      Authorization: `token ${token}`,
    },
  })
    .then((res) => res.json())