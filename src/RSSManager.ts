import { EventEmitter } from 'event';
import { type Feed, parseFeed } from 'rss';

import * as path from 'path';

import { utils } from './utils.ts';

const metaURL = new URL(import.meta.url).pathname;
const __dirname = metaURL.slice(0, metaURL.lastIndexOf('/'));

type Events = {
  subscription: [string];
  unsubscription: [string];
  newPost: [Feed];
};

export interface RSSManagerOptions {
  folder?: string;
  checkInterval?: number;
}

export class RSSManager extends EventEmitter<Events> {
  folder: string;
  feedsList: string;
  checkInterval: number;

  constructor(options?: RSSManagerOptions | null) {
    super();

    this.folder = options?.folder
      ? path.join(__dirname, options.folder)
      : path.join(__dirname, '../rss');
    this.feedsList = this.folder + '/feeds.json';
    this.checkInterval = options?.checkInterval ?? 60 * 1000;

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

  private async getSubscriptions() {
    const data = await Deno.readFile(this.feedsList),
      feeds: string[] = JSON.parse(utils.decode(data));

    return feeds;
  }

  async subscribeTo(url: string) {
    let feedsList: string[] = JSON.parse(
      utils.decode(await Deno.readFile(this.feedsList)),
    );

    if (!feedsList) {
      feedsList = [];
    }

    if (feedsList.includes(url)) {
      return Promise.reject('Already subscribed to this feed.');
    } else {
      feedsList.push(url);

      this.emit('subscription', url);
      return Deno.writeFile(
        this.feedsList,
        utils.encode(JSON.stringify(feedsList)),
      ).then(() => Promise.resolve('Added to feeds list.'));
    }
  }

  async unsubscribeFrom(url: string) {
    let feedsList: string[] = JSON.parse(
      utils.decode(await Deno.readFile(this.feedsList)),
    );

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

  startCheck() {
    return setInterval(async () => {
      const feeds = await this.getSubscriptions();

      feeds.forEach(async (feed) => {
        const resp = await fetch(feed),
          rssXML = await resp.text(),
          feedURLHostname = this.URLToHostname(feed);

        const currentXML = utils.decode(
            await Deno.readFile(
              `${this.folder}/${feedURLHostname}.xml`,
            ).catch(() => utils.encode('')),
          ),
          feedRSSXMl = await parseFeed(rssXML);

        if (currentXML.length !== rssXML.length) {
          this.emit('newPost', feedRSSXMl);
          await Deno.writeFile(
            `${this.folder}/${feedURLHostname}.xml`,
            utils.encode(rssXML),
          );
        }
      });
    }, this.checkInterval);
  }

  stopCheck(intervalId: number) {
    clearInterval(intervalId);
    return 'Stopped checking feeds.';
  }
}
