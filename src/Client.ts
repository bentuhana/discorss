import {
  ApplicationCommandInteraction,
  ApplicationCommandPartial,
  Client,
  ClientOptions,
  event,
  slash,
} from 'harmony';

export class DiscoRSSClient extends Client {
  commands: ApplicationCommandPartial[];

  constructor(options: ClientOptions, commands: ApplicationCommandPartial[]) {
    super(options);
    this.commands = commands;
  }

  @event()
  async ready() {
    console.log(
      `%cLogged in as ${this.user?.tag}!`,
      'background-color: green;',
    );

    const currentCommands = await this.interactions.commands.all();
    currentCommands.size < this.commands.length &&
      this.interactions.commands.bulkEdit(this.commands);
  }

  @slash()
  test(d: ApplicationCommandInteraction) {
    d.reply('test!');
  }

  @slash()
  test2(d: ApplicationCommandInteraction) {
    d.reply(d.option('text'));
  }
}
