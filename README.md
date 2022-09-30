# discorss

mostly hardcoded _for now_

to login with your bot, just add DISCORD_TOKEN environment variable to command.

```sh
DISCORD_TOKEN=token deno run -A src/index.ts
# or with docker
docker run -e DISCORD_TOKEN=token discorss
```

## todo

- [x] seperate commands into file or move them from Client.ts to commands.ts
- [ ] add clear feeds command
- [ ] make helper functions for things like clearOrphanFeedFiles
- [ ] command helpers
- [ ] cleanup
- [ ] maybe seperate checks into different processes?
