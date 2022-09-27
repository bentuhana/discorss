import {
  ApplicationCommandInteraction,
  ApplicationCommandPartial,
  Client,
  ClientOptions,
  customValidation,
  event,
  slash,
} from 'harmony';
import { RSSManager } from './RSSManager.ts';

import { isValidRSSFeed, isValidURL } from './utils.ts';

export class DiscoRSSClient extends Client {
  commands: ApplicationCommandPartial[];
  rssManager: RSSManager;

  constructor(options: ClientOptions, commands: ApplicationCommandPartial[]) {
    super(options);

    this.commands = commands;
    this.rssManager = new RSSManager();
  }

  @event()
  ready() {
    console.log(
      `%cLogged in as ${this.user?.tag}!`,
      'background-color: green;',
    );

    this.interactions.commands.bulkEdit(this.commands); // will update this soon

    this.rssManager.on(
      'subscription',
      (url) => console.log(`Subscribed to ${url}`),
    );
    this.rssManager.on(
      'unsubscription',
      (url) => console.log(`Unsubscribed from ${url}`),
    );
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
  @customValidation(
    (i) => isValidRSSFeed(i.option<string>('url')),
    'This URL is not an RSS feed.',
  )
  subscribe(d: ApplicationCommandInteraction) {
    const feedURL = d.option<string>('url');

    this.rssManager.subscribeTo(feedURL).then(() =>
      d.reply(`Subscribed to ${feedURL}`)
    ).catch(() => d.reply(`Already subscribed to ${feedURL}`));
  }

  @slash()
  @customValidation(
    (i) => isValidURL(i.option<string>('url')),
    'Input should be an URL.',
  )
  @customValidation(
    (i) => isValidRSSFeed(i.option<string>('url')),
    'This URL is not an RSS feed.',
  )
  unsubscribe(d: ApplicationCommandInteraction) {
    const feedURL = d.option<string>('url');

    this.rssManager.unsubscribeFrom(feedURL).then(() =>
      d.reply(`Unsubscribed from ${feedURL}`)
    ).catch(() => d.reply(`Not subscribed to ${feedURL}.`));
  }
}
