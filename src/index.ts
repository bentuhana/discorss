import { GatewayIntents } from 'harmony';
import { DiscoRSSClient } from './Client.ts';
import { commands } from './commands.ts';

new DiscoRSSClient({
  intents: [GatewayIntents.GUILDS, GatewayIntents.GUILD_MESSAGES],
}, commands).connect();
