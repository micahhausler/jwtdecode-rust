# jwtdecode

[![Build Status](https://travis-ci.org/micahhausler/jwtdecode-rust.svg?branch=master)](https://travis-ci.org/micahhausler/jwtdecode-rust)

Decodes JWT tokens


## Example

```
cat < EOF > token.jwt
eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJsc2t5d2Fsa2VyIiwiaWF0IjoyMzMzNjY0MDB9.k-tTF2CIZ-vu6-syRnCw3Zlc4jwfBCXAQRAyk0mtmso
EOF

jwtdecode token.jwt
{
  "alg": "HS256",
  "typ": "JWT"
}
{
  "sub": "lskywalker",
  "iat": 233366400
}
```

## License

MIT. See [LICENSE](/LICENSE)
