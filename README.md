# discorss

mostly hardcoded _for now_

to login with your bot, edit .env.defaults file and rename it to .env, then run:

```sh
deno run -A src/index.ts
# or with docker
docker run --env-file .env discorss
```

## todo

- [x] seperate commands into file or move them from Client.ts to commands.ts
- [ ] add clear feeds command
- [ ] make helper functions for things like clearOrphanFeedFiles
- [ ] command helpers
- [ ] cleanup
- [ ] maybe seperate checks into different processes?
