import {
  ApplicationCommandOptionType,
  ApplicationCommandPartial,
} from 'harmony';

const commands: ApplicationCommandPartial[] = [{
  name: 'latency',
  description: 'Look what gateway latency is.',
}, {
  name: 'add',
  description: 'Add RSS feed to list.',
  options: [{
    name: 'url',
    description: 'RSS feed URL',
    type: ApplicationCommandOptionType.STRING,
    required: true,
  }],
}];

export { commands };
