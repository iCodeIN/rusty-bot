const HELP_COMMAND: &str = "!help";

const HELP_MESSAGE: &str = "
    Hello there, Human!

    So you searching for help ?

    No want will help you here, you can maybe ask on the <#CHANNEL_ID> if you dare.
";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == HELP_COMMAND {
            if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected", ready.user.name);
    }
}
