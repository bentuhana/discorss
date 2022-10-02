import { type Client } from 'x/harmony';
import { DOMParser, initParser } from 'x/deno-dom';

export const validators = {
  async isValidRSSFeed(url: string) {
    const { headers } = await fetch(url),
      contentTypes = ['rss+xml', 'atom+xml', 'text/xml'],
      matches = contentTypes.filter((ct) =>
        headers.get('Content-Type')!.includes(ct)
      );

    return !!matches.length;
  },
  isValidURL(url: string) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  },
  // async isMyOwner(client: Client, senderId: string) {
  //   const ownerId = (await client.fetchApplication()).owner!.id;
  //   return ownerId === senderId ? true : false;
  // },
};

export const utils = {
  encode(input: string) {
    return new TextEncoder().encode(input);
  },
  decode(input: BufferSource) {
    return new TextDecoder().decode(input);
  },
  async htmlToText(input: string) {
    // return input.replace(/<[^>]*>?/gm, '');
    await initParser();
    return new DOMParser().parseFromString(input, 'text/html')!.textContent;
  },
};
