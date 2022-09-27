import {
  ApplicationCommandOptionType,
  ApplicationCommandPartial,
} from 'harmony';

const commands: ApplicationCommandPartial[] = [{
  name: 'latency',
  description: 'Look what gateway latency is.',
}, {
  name: 'subscribe',
  description: 'Subscribe to RSS feed.',
  options: [{
    name: 'url',
    description: 'RSS feed URL',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
}, {
  name: 'unsubscribe',
  description: 'Unsubscribe from RSS feed.',
  options: [{
    name: 'url',
    description: 'RSS feed URL',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
}];

export { commands };
