# GitHub OAuth API

It is a re-implementation of the API described by [GitHub login OAuth flow with AWS Lambda](https://www.ivarprudnikov.com/github-oauth-login-aws-lambda/) using [Cloudflare Workers](https://workers.cloudflare.com) and native Rust.

## Development

```shell
# Setup secrets first.

# GitHub OAuth Client ID
wrangler put secret GITHUB_CLIEND_ID
# GitHub OAuth Client Secret
wrangler put secret GITHUB_CLIEND_SECRET
# Callback URL for your frontend.
# Default setting is `https://your-domain.com/auth`
wrangler put secret CALLBACK_URL

wrangler dev
```

## Publish

```shell
wrangler publish
```
