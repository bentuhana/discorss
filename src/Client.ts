import { Client, ClientOptions, Embed, event } from 'x/harmony';

import { RSSManager, type RSSManagerOptions } from './RSSManager.ts';
import { Commands, commandsObject } from './commands.ts';
import { utils } from './utils.ts';

export class DiscoRSSClient extends Client {
  rssManager: RSSManager;
  rssCheckerId!: number | undefined;

  constructor(
    clientOptions: ClientOptions,
    rssManagerOptions: RSSManagerOptions,
  ) {
    super(clientOptions);

    this.rssManager = new RSSManager(rssManagerOptions);
  }

  @event()
  ready() {
    console.log(`Logged in as ${this.user?.tag}!`);

    this.interactions.commands.bulkEdit(commandsObject); // will update this soon
    this.interactions.loadModule(new Commands(this));

    if (!this.rssManager.feedPostChannelId.length) {
      throw new Error('Feed post channel Id not set.');
    } else {
      this.rssCheckerId = this.rssManager.startCheck();
      this.rssManager.on('newPost', async (post) => {
        const embed = new Embed({
          title: post?.title ?? 'No Title',
          url: post?.link ?? 'https://www.youtube.com/watch?v=dQw4w9WgXcQ',
          description: post?.description
            ? await utils.htmlToText(post.description)
            : 'No description',
          timestamp: post?.publishDate
            ? new Date(post.publishDate).toISOString()
            : new Date(Date.now()).toISOString(),
          footer: {
            text: post.categories?.join(', ') ??
              'Unknown tag',
          },
        });

        this.channels.sendMessage(this.rssManager.feedPostChannelId, embed);
      });
    }
  }
}
