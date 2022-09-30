import { type Client } from 'x/harmony';
import { DOMParser } from 'x/deno-dom';

export const validators = {
  async isValidRSSFeed(url: string) {
    const { headers } = await fetch(url);
    return headers.get('content-type')?.includes('application/rss+xml')
      ? true
      : false;
  },
  isValidURL(url: string) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  },
  async isMyOwner(client: Client, senderId: string) {
    const ownerId = (await client.fetchApplication()).owner!.id;
    return ownerId === senderId ? true : false;
  },
};

export const utils = {
  encode(input: string) {
    return new TextEncoder().encode(input);
  },
  decode(input: BufferSource) {
    return new TextDecoder().decode(input);
  },
  htmlToText(input: string) {
    return new DOMParser().parseFromString(input, 'text/html')!.textContent ||
      undefined;
  },
};
