import {
  ApplicationCommandInteraction,
  ApplicationCommandPartial,
  Client,
  ClientOptions,
  customValidation,
  event,
  slash,
} from 'harmony';

import { isValidRSSFeed, isValidURL } from './utils.ts';

export class DiscoRSSClient extends Client {
  commands: ApplicationCommandPartial[];

  constructor(options: ClientOptions, commands: ApplicationCommandPartial[]) {
    super(options);
    this.commands = commands;
  }

  @event()
  ready() {
    console.log(
      `%cLogged in as ${this.user?.tag}!`,
      'background-color: green;',
    );

    this.interactions.commands.bulkEdit(this.commands); // will update this soon
  }

  @slash()
  latency(d: ApplicationCommandInteraction) {
    d.reply(`Gateway latency: ${this.gateway.ping.toString()}`);
  }

  @slash()
  @customValidation(
    (i) => isValidURL(i.option<string>('url')),
    'Input should be an URL.',
  )
  async add(d: ApplicationCommandInteraction) {
    const URL = d.option<string>('url');
    const response = await fetch(URL);

    if (
      !isValidRSSFeed(response.headers)
    ) {
      d.reply('This URL is not an RSS feed.');
    } else {
      d.reply('Valid RSS URL!');
    }
  }
}
