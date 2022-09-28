import {
  ApplicationCommandInteraction,
  ApplicationCommandPartial,
  Client,
  ClientOptions,
  customValidation,
  event,
  slash,
} from 'harmony';

import { RSSManager, type RSSManagerOptions } from './RSSManager.ts';
import { validators } from './utils.ts';

export class DiscoRSSClient extends Client {
  commands: ApplicationCommandPartial[];
  rssManager: RSSManager;
  rssCheckerId!: number;

  constructor(
    clientOptions: ClientOptions,
    rssManagerOptions: RSSManagerOptions | null,
    commands: ApplicationCommandPartial[],
  ) {
    super(clientOptions);

    this.commands = commands;
    this.rssManager = new RSSManager(rssManagerOptions);
  }

  @event()
  ready() {
    console.log(`Logged in as ${this.user?.tag}!`);

    this.interactions.commands.bulkEdit(this.commands); // will update this soon
    this.rssCheckerId = this.rssManager.startCheck();

    this.rssManager.on(
      'newPost',
      (p) => this.channels.sendMessage('1024757600312639518', p.title.value),
    );
  }

  @slash()
  latency(d: ApplicationCommandInteraction) {
    d.reply(`Gateway latency: ${this.gateway.ping.toString()}`);
  }

  @slash()
  @customValidation(
    (i) => validators.isMyOwner(i.client, i.user.id),
    'You\'re not my author.',
  )
  async eval(i: ApplicationCommandInteraction) {
    const code = i.option<string>('code'),
      evaluated = await eval(`(async () => {${code}})()`);

    i.reply(Deno.inspect(evaluated));
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
  subscribe(i: ApplicationCommandInteraction) {
    const feedURL = i.option<string>('url');

    this.rssManager.subscribeTo(feedURL).then(() =>
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

    this.rssManager.unsubscribeFrom(feedURL).then(() =>
      i.reply(`Unsubscribed from ${feedURL}`)
    ).catch(() => i.reply(`Not subscribed to ${feedURL}.`));
  }

  @slash()
  start(i: ApplicationCommandInteraction) {
    this.rssCheckerId = this.rssManager.startCheck();
    i.reply('Started.');
  }

  @slash()
  stop(i: ApplicationCommandInteraction) {
    this.rssManager.stopCheck(this.rssCheckerId);
    i.reply('Stopped.');
  }
}
