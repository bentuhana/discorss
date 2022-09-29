import {
  ApplicationCommandOptionType,
  ApplicationCommandPartial,
} from 'harmony';

const commands: ApplicationCommandPartial[] = [{
  name: 'latency',
  description: 'Look what gateway latency is.',
}, /*{
  name: 'eval',
  description: 'Evaluate some code.',
  options: [{
    name: 'code',
    description: 'Code to be evaluated',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
},*/ {
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
}, {
  name: 'start',
  description: 'Start RSS feed',
}, {
  name: 'stop',
  description: 'Stop RSS feed',
}, {
  name: 'list',
  description: 'List subscribed RSS feeds.',
}, {
  name: 'clear',
  description: 'Clears',
  options: [{
    name: 'orphans',
    description: 'Clear orphan feed files.',
    type: ApplicationCommandOptionType.SUB_COMMAND,
  }],
}];

export { commands };
