import { EventEmitter } from 'event';
import { parseFeed } from 'rss';
import isEqual from 'isEqual';

import * as path from 'path';

import { utils } from './utils.ts';

const metaURL = new URL(import.meta.url).pathname;
const __dirname = metaURL.slice(0, metaURL.lastIndexOf('/'));

type Events = {
  subscription: [string];
  unsubscription: [string];
  newPost: [Post];
};

interface Post {
  title: string | undefined;
  author: string | undefined;
  link: string | undefined;
  publishDate: string | undefined;
  categories: (string | undefined)[] | undefined;
  description: string | undefined;
}

export interface RSSManagerOptions {
  folder?: string;
  feedChannelId?: string;
  checkInterval?: number;
}

export class RSSManager extends EventEmitter<Events> {
  folder: string;
  feedsList: string;
  checkInterval: number;
  feedPostChannelId: string;

  constructor(options?: RSSManagerOptions) {
    super();

    const feedFolder = Deno.env.get('FEED_FOLDER'),
      feedChannelId = Deno.env.get('FEED_CHANNEL_ID'),
      feedCheckInterval = Deno.env.get('FEED_CHECK_INTERVAL');

    if (feedFolder) this.folder = path.join(__dirname, feedFolder);
    else this.folder = path.join(__dirname, options?.folder ?? '../rss');
    this.feedsList = this.folder + '/feeds.json';

    if (feedCheckInterval) this.checkInterval = +feedCheckInterval * 60_000;
    else this.checkInterval = options?.checkInterval ?? 1 * 60_000;

    if (feedChannelId) this.feedPostChannelId = feedChannelId;
    else this.feedPostChannelId = options?.feedChannelId ?? '';

    Deno.stat(this.feedsList)
      .catch((err) => {
        if (err instanceof Deno.errors.NotFound) {
          Deno.mkdir(this.folder).then(() =>
            Deno.writeFile(this.feedsList, utils.encode('[]'))
          );
        }
      });
  }

  private URLToHostname = (url: string) => {
    return new URL(url).hostname;
  };

  async subscribeTo(url: string) {
    let feedsList = await this.getSubscriptions();
    if (!feedsList.length) feedsList = [];

    if (feedsList.includes(url)) {
      return Promise.reject('You are already subscribed to this feed.');
    } else {
      feedsList.push(url);

      this.emit('subscription', url);
      return Deno.writeFile(
        this.feedsList,
        utils.encode(JSON.stringify(feedsList)),
      ).then(() => Promise.resolve('Subscribed to feed.'));
    }
  }

  async unsubscribeFrom(url: string) {
    let feedsList = await this.getSubscriptions();

    if (!feedsList || !feedsList.includes(url)) {
      return Promise.reject('Not subscribed to this feed');
    } else {
      feedsList = feedsList.filter((el: string) => el !== url);

      this.emit('unsubscription', url);
      return Deno.writeFile(
        this.feedsList,
        utils.encode(JSON.stringify(feedsList)),
      ).then(() => Promise.resolve('Removed feed from list.'));
    }
  }

  async getSubscriptions() {
    const data = await Deno.readFile(this.feedsList),
      feeds: string[] = JSON.parse(utils.decode(data));

    return feeds;
  }

  startCheck() {
    return setInterval(async () => {
      const subscriptions = await this.getSubscriptions();

      subscriptions.forEach(async (feedURL) => {
        const feedURLAsHostname = this.URLToHostname(feedURL);

        const resp = await fetch(feedURL),
          feedRSS = await resp.text(),
          lastEntry = (await parseFeed(feedRSS)).entries[0];

        const postJSON = {
          title: lastEntry.title?.value ?? undefined,
          author: lastEntry.author?.name ?? undefined,
          link: lastEntry.links[0]?.href ?? undefined,
          publishDate: lastEntry?.publishedRaw ?? undefined,
          categories: lastEntry.categories
            ? lastEntry.categories.map((ctg) => ctg.term)
            : undefined,
          description: lastEntry.description?.value,
        };
        let postFileJSON;

        try {
          postFileJSON = JSON.parse(
            utils.decode(
              await Deno.readFile(`${this.folder}/${feedURLAsHostname}.json`),
            ),
          );
        } catch (err) {
          if (err instanceof Deno.errors.NotFound) {
            await Deno.writeFile(
              `${this.folder}/${feedURLAsHostname}.json`,
              utils.encode(JSON.stringify(postJSON)),
            );
          }
        }

        if (!isEqual(postFileJSON, postJSON)) {
          Deno.writeFile(
            `${this.folder}/${feedURLAsHostname}.json`,
            utils.encode(JSON.stringify(postJSON)),
          );
          this.emit('newPost', postJSON);
        }
      });
    }, this.checkInterval);
  }

  stopCheck(intervalId: number) {
    clearInterval(intervalId);
  }

  async clearOrphanFeedFiles() {
    const currentFeedsList = (await this.getSubscriptions()).map((f) =>
        this.URLToHostname(f)
      ),
      createdFeedFiles = [];

    for await (const feedFile of Deno.readDir(this.folder)) {
      if (feedFile.name === 'feeds.json') continue;
      createdFeedFiles.push(
        feedFile.name.substring(0, feedFile.name.lastIndexOf('.')),
      );
    }

    const orphans = createdFeedFiles.filter((cf) =>
      !currentFeedsList.includes(cf)
    );

    if (!orphans.length) {
      return Promise.reject('No orphan file found.');
    } else {
      orphans.forEach(async (orphan) => {
        await Deno.remove(`${this.folder}/${orphan}.json`);
      });

      return Promise.resolve('Cleared orphan files.');
    }
  }
}
