import {
  ApplicationCommandOptionType,
  ApplicationCommandPartial,
} from 'harmony';

const commands: ApplicationCommandPartial[] = [{
  name: 'test',
  description: 'test',
}, {
  name: 'test2',
  description: 'test2',
  options: [{
    name: 'text',
    description: 'text',
    required: true,
    type: ApplicationCommandOptionType.STRING,
  }],
}];

export { commands };
