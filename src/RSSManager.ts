import { EventEmitter } from 'event';
import { type Feed } from 'rss';

import * as path from 'path';

const metaURL = new URL(import.meta.url).pathname;
const __dirname = metaURL.slice(0, metaURL.lastIndexOf('/'));

const encoder = new TextEncoder();
const decoder = new TextDecoder();

type Events = {
  subscription: [string];
  unsubscription: [string];
  newPost: [Feed];
};

export class RSSManager extends EventEmitter<Events> {
  folder: string;
  feedsList: string;
  checkInterval: number;

  constructor(options?: {
    folder?: string;
    checkInterval?: number;
  }) {
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
            Deno.writeFile(this.feedsList, encoder.encode('{}'))
          );
        }
      });
  }

  private URLToHostname = (url: string) => {
    const trimURLRegEx = /https?:\/\/([-a-zA-Z0-9@:%._\+~#=]{1,256})\/?/; // i suck at regex
    return trimURLRegEx.exec(url)?.[1];
  };

  async subscribeTo(url: string) {
    const feedsListJSON = JSON.parse(
      decoder.decode(await Deno.readFile(this.feedsList)),
    );

    if (!feedsListJSON['feeds']) {
      feedsListJSON['feeds'] = [];
    }

    if (feedsListJSON['feeds'].includes(url)) {
      return Promise.reject('Already subscribed to this feed.');
    } else {
      feedsListJSON['feeds'].push(url);

      this.emit('subscription', url);
      return Deno.writeFile(
        this.feedsList,
        encoder.encode(JSON.stringify(feedsListJSON, null, 2)),
      ).then(() => Promise.resolve('Added to feeds list.'));
    }
  }

  async unsubscribeFrom(url: string) {
    const feedsListJSON = JSON.parse(
      decoder.decode(await Deno.readFile(this.feedsList)),
    );

    if (!feedsListJSON['feeds'] || !feedsListJSON['feeds'].includes(url)) {
      return Promise.reject('Not subscribed to this feed');
    } else {
      feedsListJSON['feeds'] = feedsListJSON['feeds'].filter((el: string) =>
        el !== url
      );

      this.emit('unsubscription', url);
      return Deno.writeFile(
        this.feedsList,
        encoder.encode(JSON.stringify(feedsListJSON, null, 2)),
      ).then(() => Promise.resolve('Removed feed from list.'));
    }
  }
}
