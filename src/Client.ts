import {
  ApplicationCommandInteraction,
  ApplicationCommandPartial,
  Client,
  ClientOptions,
  customValidation,
  Embed,
  event,
  slash,
  subslash,
} from 'harmony';

import { RSSManager, type RSSManagerOptions } from './RSSManager.ts';
import { utils, validators } from './utils.ts';

export class DiscoRSSClient extends Client {
  commands: ApplicationCommandPartial[];
  rssManager: RSSManager;
  rssCheckerId!: number;

  constructor(
    clientOptions: ClientOptions,
    rssManagerOptions: RSSManagerOptions,
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

    if (!this.rssManager.feedPostChannelId.length) {
      throw new Error('Feed post channel Id not set.');
    } else {
      this.rssCheckerId = this.rssManager.startCheck();
      this.rssManager.on('newPost', (post) => {
        const embed = new Embed({
          title: post?.title ?? 'No Title',
          url: post?.link ?? 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
          description: post?.description
            ? utils.htmlToText(post.description)
            : 'No description',
          timestamp: post?.publishDate
            ? new Date(post.publishDate).toISOString()
            : undefined,
          footer: {
            text: post.categories?.join(', ') ??
              'Unknown tag',
          },
        });

        this.channels.sendMessage(this.rssManager.feedPostChannelId, embed);
      });
    }
  }

  @slash()
  latency(d: ApplicationCommandInteraction) {
    d.reply(`Gateway latency: ${this.gateway.ping.toString()}`);
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

  @slash()
  async list(i: ApplicationCommandInteraction) {
    const feedsList = await this.rssManager.getSubscriptions();
    i.reply(feedsList.join(', '));
  }

  @subslash('clear')
  orphans(i: ApplicationCommandInteraction) {
    this.rssManager.clearOrphanFeedFiles().then(() =>
      i.reply('Cleared orphan files.')
    ).catch(() => i.reply('No orphan file found.'));
  }
}
