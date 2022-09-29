import 'dotenv';
import { GatewayIntents } from 'harmony';

import { DiscoRSSClient } from './Client.ts';

new DiscoRSSClient(
  {
    intents: [GatewayIntents.GUILDS],
  },
  {},
).connect();
