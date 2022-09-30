import 'std/dotenv/load';
import { GatewayIntents } from 'x/harmony';

import { DiscoRSSClient } from './Client.ts';

new DiscoRSSClient(
  {
    intents: [GatewayIntents.GUILDS],
  },
  {},
).connect();
