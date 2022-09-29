import {
  ApplicationCommandInteraction,
  ApplicationCommandOptionType,
  ApplicationCommandPartial,
  ApplicationCommandsModule,
  customValidation,
  slash,
  subslash,
} from 'harmony';

import { DiscoRSSClient } from './Client.ts';
import { validators } from './utils.ts';

export const commandsObject: ApplicationCommandPartial[] = [{
  name: 'latency',
  description: 'Look what gateway latency is.',
}, /*{
  name: 'eval',
  description: 'Evaluate some code.',
  options: [{
    name: 'code',
    description: 'Code to be evaluated',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
},*/ {
  name: 'subscribe',
  description: 'Subscribe to RSS feed.',
  options: [{
    name: 'url',
    description: 'RSS feed URL',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
}, {
  name: 'unsubscribe',
  description: 'Unsubscribe from RSS feed.',
  options: [{
    name: 'url',
    description: 'RSS feed URL',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
}, {
  name: 'start',
  description: 'Start RSS feed',
}, {
  name: 'stop',
  description: 'Stop RSS feed',
}, {
  name: 'list',
  description: 'List subscribed RSS feeds.',
}, {
  name: 'clear',
  description: 'Clears',
  options: [{
    name: 'orphans',
    description: 'Clear orphan feed files.',
    type: ApplicationCommandOptionType.SUB_COMMAND,
  }],
}];

export class Commands extends ApplicationCommandsModule {
  client: DiscoRSSClient;

  constructor(client: DiscoRSSClient) {
    super();
    this.client = client;
  }

  @slash()
  latency(d: ApplicationCommandInteraction) {
    d.reply(`Gateway latency: ${this.client.gateway.ping.toString()}`);
  }

  // @slash()
  // @customValidation(
  //   (i) => validators.isMyOwner(i.client, i.user.id),
  //   'You\'re not my author.',
  // )
  // async eval(i: ApplicationCommandInteraction) {
  //   const code = i.option<string>('code'),
  //     evaluated = await eval(`(async () => {${code}})()`);

  //   i.reply(Deno.inspect(evaluated));
  // }

  @slash()
  @customValidation(
    (i) => validators.isValidURL(i.option<string>('url')),
    'Input should be an URL.',
  )
  @customValidation(
    (i) => validators.isValidRSSFeed(i.option<string>('url')),
    'This URL is not an RSS feed.',
  )
  subscribe(i: ApplicationCommandInteraction) {
    const feedURL = i.option<string>('url');

    this.client.rssManager.subscribeTo(feedURL).then(() =>
      i.reply(`Subscribed to ${feedURL}`)
    ).catch(() => i.reply(`Already subscribed to ${feedURL}`));
  }

  @slash()
  @customValidation(
    (i) => validators.isValidURL(i.option<string>('url')),
    'Input should be an URL.',
  )
  @customValidation(
    (i) => validators.isValidRSSFeed(i.option<string>('url')),
    'This URL is not an RSS feed.',
  )
  unsubscribe(i: ApplicationCommandInteraction) {
    const feedURL = i.option<string>('url');

    this.client.rssManager.unsubscribeFrom(feedURL).then(() =>
      i.reply(`Unsubscribed from ${feedURL}`)
    ).catch(() => i.reply(`Not subscribed to ${feedURL}.`));
  }

  @slash()
  start(i: ApplicationCommandInteraction) {
    if (this.client.rssCheckerId) i.reply('Already started.');
    else {
      this.client.rssCheckerId = this.client.rssManager.startCheck();
      i.reply('Started.');
    }
  }

  @slash()
  stop(i: ApplicationCommandInteraction) {
    if (!this.client.rssCheckerId) i.reply('Already stopped.');
    else {
      this.client.rssManager.stopCheck(this.client.rssCheckerId);
      this.client.rssCheckerId = undefined;

      i.reply('Stopped.');
    }
  }

  @slash()
  async list(i: ApplicationCommandInteraction) {
    const feedsList = await this.client.rssManager.getSubscriptions();
    i.reply(feedsList.join(', '));
  }

  @subslash('clear')
  orphans(i: ApplicationCommandInteraction) {
    this.client.rssManager.clearOrphanFeedFiles().then(() =>
      i.reply('Cleared orphan files.')
    ).catch(() => i.reply('No orphan file found.'));
  }
}
