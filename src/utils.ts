export const validators = {
  async isValidRSSFeed(url: string) {
    const { headers } = await fetch(url);
    return headers.get('content-type')?.includes('application/rss+xml')
      ? true
      : false;
  },
  isValidURL(url: string) {
    const URLRegEx =
      /https?:\/\/(www\.)?[-a-zA-Z0-9@:%._\+~#=]{1,256}\.[a-zA-Z0-9()]{1,6}\b([-a-zA-Z0-9()!@:%_\+.~#?&\/\/=]*)/i;

    return URLRegEx.test(url) ? true : false;
  },
};
